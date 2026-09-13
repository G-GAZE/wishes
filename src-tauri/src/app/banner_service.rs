//! # 卡池管理和抽卡总服务
//! 
//! 提供卡池管理、抽卡执行、状态持久化和卡池相关信息查询等功能.
//! 该服务是应用层的核心, 协调注册器、逻辑引擎和状态数据库完成一次抽卡流程.

use std::{hash::Hasher, sync::Arc};
use anyhow::{Context, Result};
use rand::{SeedableRng, rngs::ChaCha12Rng, seq::IndexedRandom};
use crate::{
    app::logic_engine::LogicEngine,
    domain::{ids::{BannerId, UserId}, wish_result::WishResult},
    infrastructure::{
        registry::{BannerRegistry, CardRegistry, DeckRegistry, LogicRegistry},
        repository::state_repo::StateRepository
    },
    interface::banner_info::{BannerInfo, BannerSummary}
};


/// 卡池管理和抽卡总服务
/// 
/// 持有注册器、逻辑引擎和状态数据库的 `Arc` 引用, 提供:
/// - 执行单次抽卡 (`wish`)
/// - 加载/保存卡池逻辑状态
/// - 查询卡池摘要和详细信息
pub struct BannerService {
    /// 卡池注册器引用.
    card_registry: Arc<CardRegistry>,
    /// 卡组注册器引用.
    deck_registry: Arc<DeckRegistry>,
    /// 逻辑注册器引用.
    logic_registry: Arc<LogicRegistry>,
    /// 卡池注册器引用.
    banner_registry: Arc<BannerRegistry>,
    /// 逻辑引擎引用.
    logic_engine: Arc<LogicEngine>,
    /// 逻辑状态数据库引用.
    state_repository: Arc<StateRepository>,
}

impl BannerService {
    /// 创建新的卡池总服务.
    /// 
    /// 所有参数均为 `Arc` 引用, 以便在多个服务间共享.
    pub fn new(
        card_registry: Arc<CardRegistry>,
        deck_registry: Arc<DeckRegistry>,
        logic_registry: Arc<LogicRegistry>,
        banner_registry: Arc<BannerRegistry>,
        logic_engine: Arc<LogicEngine>,
        state_repository: Arc<StateRepository>,
    ) -> Self {
        Self {
            card_registry,
            deck_registry,
            logic_registry,
            banner_registry,
            logic_engine,
            state_repository
        }
    }

    /// 执行单次抽卡.
    /// 
    /// # 流程
    /// 1. 根据 `banner_id` 获取卡池 (加锁).
    /// 2. 从卡池逻辑状态中读取 `total_counter`, 并用其构建随机种子.
    /// 3. 调用逻辑引擎执行抽卡逻辑, 得到逻辑结果 `LogicResult`.
    /// 4. 根据逻辑结果包含的标签描述从对应卡组中查询候选卡片.
    /// 5. 在所有候选卡片中等概率随机选取一张.
    /// 6. 更新卡池逻辑状态中的 `total_counter` 并持久化到数据库.
    /// 7. 返回抽卡结果 `WishResult`, 包含标签化的卡片和活动标签.
    /// 
    /// # 错误
    /// - 卡池、卡组不存在.
    /// - 逻辑执行时发生错误.
    /// - 候选卡片列表为空 (说明卡组未覆盖逻辑的所有可能输出).
    /// - 等概率随机选取失败.
    /// - 状态持久化失败.
    pub fn wish(&self, banner_id: BannerId) -> Result<WishResult> {
        let banner_lock = self.banner_registry.get(banner_id)
            .ok_or_else(|| anyhow::anyhow!("未找到 Banner: Id `{}`", banner_id.0))?;
        let mut tagged_banner = banner_lock.lock();
        
        let total_counter = tagged_banner.inner.logic_instance.state.get("total_counter")
            .and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        let seed = self.build_seed("0", banner_id, total_counter);  // profile 固定为 0, TODO: 后期添加多用户
        let mut rng = ChaCha12Rng::seed_from_u64(seed);

        let result = self.logic_engine.execute(
            tagged_banner.inner.logic_instance.logic_id,
            &mut tagged_banner.inner.logic_instance,
            &mut rng
        )?;

        let tagged_deck = self.deck_registry.get(tagged_banner.inner.deck_id)
            .ok_or_else(|| anyhow::anyhow!("未找到 Deck: Id `{}`", tagged_banner.inner.deck_id.0))?;

        let candidates = tagged_deck.inner.query_cards(
            &self.card_registry,
            &result.tags,
            &result.event_tags,
        );

        if candidates.is_empty() {
            let member_count = tagged_deck.members.resolve(&self.card_registry).len();
            tracing::error!(
                banner_id = %tagged_banner.id.0,
                deck_id = %tagged_deck.id.0,
                logic_id = %tagged_banner.logic_instance.logic_id.0,
                tags = ?result.tags,
                event_tags = ?result.event_tags,
                member_count = %member_count,
                "Deck {} 没有 Logic {} 匹配 Tag {:?} 和 EventTag {:?} 的 Card, 当前 Deck 成员数量: {}",
                tagged_deck.inner.id.0,
                tagged_banner.inner.logic_instance.logic_id.0,
                result.tags,
                result.event_tags,
                member_count,
            );
            anyhow::bail!(
                "Deck {} 没有 Logic {} 匹配 Tag {:?} 和 EventTag {:?} 的 Card, 当前 Deck 成员数量: {}",
                tagged_deck.inner.id.0,
                tagged_banner.inner.logic_instance.logic_id.0,
                result.tags,
                result.event_tags,
                member_count,
            );
        }

        let picked_id = candidates.choose(&mut rng)
            .ok_or_else(|| anyhow::anyhow!("随机抽取 CardId 失败!"))?;

        tagged_banner.inner.logic_instance.state["total_counter"] = serde_json::Value::Number((total_counter + 1).into());

        self.state_repository.save(UserId(0), banner_id, &tagged_banner.inner.logic_instance.state)
            .with_context(|| format!("Banner {} 存储逻辑状态时失败", banner_id.0))?;

        let tagged_card = self.card_registry.get(*picked_id)
            .ok_or_else(|| anyhow::anyhow!("未找到 Card: Id `{}`", picked_id.0))?;

        Ok(WishResult {
            card: tagged_card,
            event_tags: result.event_tags.iter().cloned().collect(),
        })
    }

    /// 从指定用户 (profile)、卡池和抽卡次数构建确定性随机种子.
    /// 
    /// 使用 `std::collections::hash_map::DefaultHasher` 将字符串、Id 和计数器混合, 确保同一参数下产生相同随机种子.
    /// 当前 `profile` 固定为 "0" (单用户模式).
    pub fn build_seed(&self, profile: &str, banner_id: BannerId, total_counter: u32) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        hasher.write(profile.as_bytes());
        hasher.write_u64(banner_id.0);
        hasher.write_u32(total_counter);
        hasher.finish()
    }

    /// 从数据库加载所有卡池的逻辑状态.
    /// 
    /// 遍历 `banner_registry` 中的所有卡池, 尝试从 `state_repository` 加载状态.
    /// 若存在则更新到卡池的 `logic_instance.state` 中.
    pub fn load_banner_state(&self) -> Result<()> {
        for banner_id in self.banner_registry.ids() {
            let banner_lock = self.banner_registry.get(banner_id)
                .ok_or_else(|| anyhow::anyhow!("未找到 Banner: Id `{}`", banner_id.0))?;
            let mut tagged_banner = banner_lock.lock();

            if let Ok(Some(state_json)) = self.state_repository.load(UserId(0), banner_id) {
                tagged_banner.inner.logic_instance.state = state_json;
            }
        }
        Ok(())
    }

    /// 获取所有卡池的摘要信息列表 `BannerSummary`, 按 Id 排序.
    pub fn get_banner_summaries(&self) -> Vec<BannerSummary> {
        let mut summaries: Vec<_> = self.banner_registry
            .ids()
            .iter()
            .filter_map(|id| {
                let banner_lock = self.banner_registry.get(*id)?;
                let banner = banner_lock.lock();
                Some(BannerSummary {
                    id: banner.inner.id.0,
                    name: banner.inner.name.clone(),
                    tags: banner.tags.iter().cloned().collect(),
                })
            })
            .collect();
        summaries.sort_by_key(|s| s.id);    // 按 BannerId 排序, 确保前端有序
        summaries
    }

    /// 获取指定卡池的详细信息 `BannerInfo`, 按 Id 排序.
    pub fn get_banner_info(&self, banner_id: BannerId) -> Result<BannerInfo> {
        let banner_lock = self.banner_registry.get(banner_id)
            .ok_or_else(|| anyhow::anyhow!("未找到 Banner: id {}", banner_id.0))?;
        let banner = banner_lock.lock();

        let state = &banner.inner.logic_instance.state;
        let total_counter = state.get("total_counter")
            .and_then(|v| v.as_u64()).unwrap_or(0) as u32;

        let deck = self.deck_registry.get(banner.inner.deck_id)
            .ok_or_else(|| anyhow::anyhow!("未找到 Deck: Id {}", banner.inner.deck_id.0))?;
        let deck_name = deck.inner.name.clone();

        let logic_def = self.logic_registry.get_definition(banner.inner.logic_instance.logic_id)
            .ok_or_else(|| anyhow::anyhow!("未找到 Logic: Id {}", banner.inner.logic_instance.logic_id.0))?;
        let logic_name = logic_def.inner.name.clone();

        Ok(BannerInfo {
            id: banner.inner.id.0,
            name: banner.inner.name.clone(),
            tags: banner.tags.iter().cloned().collect(),
            deck_name,
            logic_name,
            total_counter,
        })
    }
}