//! # 数据加载器
//! 
//! 负责从文件系统加载各类数据 (卡片、卡组、逻辑、卡池) 并构建注册器.
//! 加载过程中会进行引用完整性和唯一性检验, 并对卡池进行逻辑到卡组的标签覆盖性检验.

use std::{fs, path::Path};
use anyhow::{Context, Result};
use walkdir::WalkDir;
use crate::{
    domain::{
        banner::TaggedBanner,
        card::TaggedCard,
        deck::TaggedDeck,
        logic::definition::{
            LogicVariant,
            TaggedLogicDefinition
        }
    },
    infrastructure::registry::{
        BannerRegistry,
        CardRegistry,
        DeckRegistry,
        LogicRegistry
    }
};

/// 加载器.
/// 所有方法均为静态风格.
pub struct Loader;

impl Loader {
    /// 创建一个新的加载器.
    pub fn new() -> Self {
        Self {}
    }

    /// 从指定目录及其子目录加载所有 JSON 文件作为卡片定义, 注册至 `CardRegistry` 并返回.
    /// 
    /// # 校验
    /// - 每个卡片的 Id 必须唯一.
    /// - 若发现重复 Id 则返回错误.
    pub fn load_cards_from_dir(&self, dir_path: &Path) -> Result<CardRegistry> {
        let card_registry = CardRegistry::new();
        let mut max_id = 0;

        for entry in WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
        {
            let path = entry.path();
            let data = fs::read_to_string(path)
                .with_context(|| format!("无法读取 Card 定义文件 - file: {}", path.display()))?;
            let tagged_card: TaggedCard = serde_json::from_str(&data)
                .with_context(|| format!("解析 Card 文件中的 JSON 数据失败 - file: {}", path.display()))?;

            let id = tagged_card.inner.id;      // id 唯一性检查
            if card_registry.contains(id) {
                anyhow::bail!("重复声明的 Card Id `{}` - file: {}", id.0, path.display())
            }

            if id.0 > max_id {                          // 获取最大 Id
                max_id = id.0;
            }

            let tags: Vec<_> = tagged_card.tags.iter().cloned().collect();
            card_registry.tag_index().insert(id, &tags);
            card_registry.insert(tagged_card);
            card_registry.insert_path(id, path.to_path_buf());
        }

        card_registry.reset_id(max_id);

        Ok(card_registry)
    }

    /// 从指定目录及其子目录加载所有 JSON 文件作为卡组定义, 注册至 `DeckRegistry` 并返回.
    /// 
    /// # 校验
    /// - 卡组 Id 唯一.
    /// - 卡组中引用的卡片存在于 `card_registry` 注册器中.
    pub fn load_decks_from_dir(&self, dir_path: &Path, card_registry: &CardRegistry) -> Result<DeckRegistry> {
        let deck_registry = DeckRegistry::new();

        for entry in WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
        {
            let path = entry.path();
            let data = fs::read_to_string(path)
                .with_context(|| format!("无法读取 Deck 定义文件 - file: {}", path.display()))?;
            let tagged_deck: TaggedDeck = serde_json::from_str(&data)
                .with_context(|| format!("解析 Deck 文件中的 JSON 数据失败 - file: {}", path.display()))?;

            let id = tagged_deck.inner.id;        // id 唯一性检查
            if deck_registry.contains(id) {
                anyhow::bail!("重复声明的 Deck Id `{}` - file: {}", id.0, path.display())
            }

            for &card_id in &tagged_deck.inner.members {        // 检查 members
                if !card_registry.contains(card_id) {
                    anyhow::bail!(
                        "Deck {} 的 members 中引用了未定义的 Card(id: {}) - file: {}",
                        tagged_deck.inner.id.0, card_id.0, path.display()
                    );
                }
            }
            for (event_tag, group_members) in &tagged_deck.inner.event_groups {
                for &card_id in group_members {                 // 检查 event_groups
                    if !card_registry.contains(card_id) {
                        anyhow::bail!(
                            "Deck {} 的 Event Group {:?} 引用了未定义的 Card(id: {}) - file: {}", 
                            tagged_deck.inner.id.0, event_tag, card_id.0, path.display()
                        );
                    }
                }
            }

            let tags: Vec<_> = tagged_deck.tags.iter().cloned().collect();
            deck_registry.tag_index.insert(id, &tags);
            deck_registry.insert(tagged_deck);
            // TODO: 添加路径
            
        }

        Ok(deck_registry)
    }

    /// 从指定目录及其子目录加载所有 JSON 文件作为逻辑定义, 并注册至传入的 `logic_registry`.
    /// 
    /// # 校验
    /// - 逻辑 Id 唯一.
    /// - 如果逻辑变体为 `LogicVariant::Hardcoded`, 则引用的执行器名称必须已注册.
    /// 
    /// # 注意
    /// 此方法需要修改传入的 `logic_registry`.
    pub fn load_logics_from_dir(&self, dir_path: &Path, logic_registry: &mut LogicRegistry) -> Result<()> {
        for entry in WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
        {
            let path = entry.path();
            let data = fs::read_to_string(path)
                .with_context(|| format!("无法读取 Logic 定义文件 - file: {}", path.display()))?;
            let tagged_logic_def: TaggedLogicDefinition = serde_json::from_str(&data)
                .with_context(|| format!("解析 Logic 文件中的 JSON 数据失败 - file: {}", path.display()))?;

            let id = tagged_logic_def.inner.id;        // id 唯一性检查
            if logic_registry.contains_definition(id) {
                anyhow::bail!("重复声明的 Logic Id `{}` - file: {}", id.0, path.display())
            }

            if let LogicVariant::Hardcoded { executor_name } = &tagged_logic_def.inner.variant {
                if !logic_registry.contains_hardcoded_executor(executor_name) {
                    anyhow::bail!(
                        "Logic {} 引用了未注册的硬编码执行器 (Hardcoded Executor) {} - file: {}",
                        tagged_logic_def.inner.id.0, executor_name, path.display()
                    )
                }
            }
            
            let tags: Vec<_> = tagged_logic_def.tags.iter().cloned().collect();
            logic_registry.tag_index.insert(id, &tags);
            logic_registry.insert_definition(tagged_logic_def);
            // 添加路径

        }

        Ok(())
    }

    /// 从指定目录及其子目录加载所有 JSON 文件作为卡池定义, 注册至 `BannerRegistry` 中并返回.
    /// 
    /// # 校验
    /// - 卡组 Id 唯一.
    /// - 引用的逻辑定义和卡组必须存在.
    /// - 如果逻辑为 `LogicVariant::Hardcoded`, 将检查卡组是否覆盖了所有可能的输出标签组合.
    pub fn load_banner_from_dir(
        &self,
        dir_path: &Path,
        card_registry: &CardRegistry,
        deck_registry: &DeckRegistry,
        logic_registry: &LogicRegistry
    ) -> Result<BannerRegistry> {
        let banner_registry = BannerRegistry::new();
        
        for entry in WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
        {
            let path = entry.path();
            let data = fs::read_to_string(path)
                .with_context(|| format!("无法读取 Banner 定义文件 - file: {}", path.display()))?;
            let tagged_banner: TaggedBanner = serde_json::from_str(&data)
                .with_context(|| format!("解析 Banner 文件中的 JSON 数据失败 - file: {}", path.display()))?;
            
            let id = tagged_banner.inner.id;        // id 唯一性检查
            if banner_registry.contains(id) {
                anyhow::bail!("重复声明的 Banner Id `{}` - file: {}", id.0, path.display())
            }

            // 检查 Logic
            if !logic_registry.contains_definition(tagged_banner.inner.logic_instance.logic_id) {
                anyhow::bail!(
                    "Banner {} 引用了未定义的 Logic {} - file: {}",
                    tagged_banner.inner.id.0, tagged_banner.inner.logic_instance.logic_id.0, path.display()
                );
            }

            // 检查 Deck
            if let Some(tagged_deck) = deck_registry.get(tagged_banner.inner.deck_id) {
                let tagged_logic_def = logic_registry.get_definition(tagged_banner.inner.logic_instance.logic_id).unwrap();
                if let LogicVariant::Hardcoded { executor_name } = &tagged_logic_def.inner.variant {
                    let mut missing = Vec::new();
                    let combos = logic_registry.hardcoded_possible_output_combinations(executor_name)
                        .unwrap_or_else(|| Vec::new());
                    for combo in combos {
                        let candidates = tagged_deck.inner.query_cards(card_registry, &combo.0, &combo.1);
                        if candidates.is_empty() {
                            missing.push(combo);
                        }
                    }
                    if !missing.is_empty() {
                        anyhow::bail!(
                            "Banner {} 中的 Deck {} 缺少 Logic {} 可能输出的 Tag 组合 - file: {}\n{}",
                            tagged_banner.inner.id.0, tagged_banner.inner.id.0, tagged_logic_def.inner.id.0, path.display(),
                            missing.iter()
                                .map(|p| format!("  - {:?}", p))
                                .collect::<Vec<_>>()
                                .join("\n")
                        );
                    }
                }

                let tags: Vec<_> = tagged_banner.tags.iter().cloned().collect();
                banner_registry.tag_index.insert(id, &tags);
                banner_registry.insert(tagged_banner);
                // 添加路径

            } else {
                anyhow::bail!(
                    "Banner {} 引用了未定义的 Deck {} - file: {}",
                    tagged_banner.inner.id.0, tagged_banner.inner.deck_id.0, path.display()
                );
            }
        }

        Ok(banner_registry)
    }
}