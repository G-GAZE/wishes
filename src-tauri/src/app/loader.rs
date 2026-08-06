use std::{fs, path::Path};

use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::{domain::{banner::TaggedBanner, card::TaggedCard, deck::TaggedDeck, logic::definition::{LogicVariant, TaggedLogicDefinition}}, infrastructure::registry::{BannerRegistry, CardRegistry, DeckRegistry, LogicRegistry}};



pub struct Loader;

impl Loader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load_cards_from_dir(&self, dir_path: &Path) -> Result<CardRegistry> {
        let card_registry = CardRegistry::new();

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

            card_registry.tag_index().insert(id, &tagged_card.tags);
            card_registry.insert(tagged_card);
            card_registry.insert_path(id, path.to_path_buf());
        }

        Ok(card_registry)
    }

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

            deck_registry.tag_index.insert(id, &tagged_deck.tags);
            deck_registry.insert(tagged_deck);
            // TODO: 添加路径
            
        }

        Ok(deck_registry)
    }

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

            logic_registry.tag_index.insert(id, &tagged_logic_def.tags);
            logic_registry.insert_definition(tagged_logic_def);
            // 添加路径

        }

        Ok(())
    }

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

                banner_registry.tag_index.insert(id, &tagged_banner.tags);
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