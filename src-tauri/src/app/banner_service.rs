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
pub struct BannerService {
    card_registry: Arc<CardRegistry>,
    deck_registry: Arc<DeckRegistry>,
    logic_registry: Arc<LogicRegistry>,
    banner_registry: Arc<BannerRegistry>,
    
    logic_engine: Arc<LogicEngine>,
    state_repository: Arc<StateRepository>,
}


impl BannerService {
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
            anyhow::bail!(
                "Deck {} 没有匹配 Tag {:?} 和 EventTag {:?} 的 Card, 当前 Deck 成员数量: {}",
                tagged_deck.inner.id.0,
                result.tags,
                result.event_tags,
                tagged_deck.inner.members.len()
            )
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

    pub fn build_seed(&self, profile: &str, banner_id: BannerId, total_counter: u32) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        hasher.write(profile.as_bytes());
        hasher.write_u64(banner_id.0);
        hasher.write_u32(total_counter);
        hasher.finish()
    }

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