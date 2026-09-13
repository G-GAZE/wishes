use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

use anyhow::{Context, Result};

use crate::{app::validation, domain::{deck::{Deck, EventGroup, Membership, TaggedDeck}, ids::DeckId, tag::{EventTag, Tag, Tagged}}, infrastructure::registry::{BannerRegistry, CardRegistry, DeckRegistry, LogicRegistry}};



pub struct DeckManager {
    deck_registry: Arc<DeckRegistry>,
    card_registry: Arc<CardRegistry>,       // 其他注册器用于卡组修改/删除时的数据校验
    banner_registry: Arc<BannerRegistry>,
    logic_registry: Arc<LogicRegistry>,
    base_path: PathBuf,
}

impl DeckManager {
    pub fn new(
        deck_registry: Arc<DeckRegistry>,
        card_registry: Arc<CardRegistry>,
        banner_registry: Arc<BannerRegistry>,
        logic_registry: Arc<LogicRegistry>,
        base_path: PathBuf,
    ) -> Self {
        Self {
            deck_registry,
            card_registry,
            banner_registry,
            logic_registry,
            base_path
        }
    }

    pub fn list_all(&self) -> Vec<Arc<TaggedDeck>> {
        self.deck_registry.all_decks()
    }

    pub fn list_by_tags(&self, tags: &[Tag]) -> Vec<Arc<TaggedDeck>> {
        let ids = self.deck_registry.tag_index.query(tags);
        ids.into_iter()
            .filter_map(|id| self.deck_registry.get(id))
            .collect()
    }

    pub fn create_deck(&self,
        name: String,
        members: Membership,
        event_groups: HashMap<EventTag, EventGroup>,
        tags: Vec<Tag>
    ) -> Result<TaggedDeck> {
        let id = self.deck_registry.allocate_id();

        let deck = Deck {
            id,
            name,
            members,
            event_groups,
        };
        let tagged_deck = Tagged::with_tags(deck, tags.into_iter().collect());

        let file_path = self.build_file_path(&tagged_deck);
        Self::save_to_path(&tagged_deck, &file_path)?;

        self.deck_registry.insert(tagged_deck.clone());
        self.deck_registry.insert_path(id, file_path);

        Ok(tagged_deck)
    }

    pub fn update_deck(&self,
        id: DeckId,
        name: Option<String>,
        members: Option<Membership>,
        event_groups: Option<HashMap<EventTag, EventGroup>>,
        tags: Option<Vec<Tag>>,
    ) -> Result<TaggedDeck> {
        let old = self.deck_registry.get(id)
            .ok_or_else(|| anyhow::anyhow!("Deck {} 不存在", id.0))?;
        let old_path_opt = self.deck_registry.get_path(id);      // 允许旧文件不存在

        let mut new_deck = old.as_ref().clone();
        if let Some(name) = name { new_deck.inner.name = name; }
        if let Some(members) = members { new_deck.inner.members = members; }
        if let Some(event_groups) = event_groups { new_deck.inner.event_groups = event_groups; }
        if let Some(tags) = tags { new_deck.tags = tags.into_iter().collect(); }

        // 校验卡组更新后的数据完整性
        let affected_banners = self.banner_registry.find_banners_by_deck(id);
        for banner_id in affected_banners {
            let banner_lock = self.banner_registry.get(banner_id)
                .ok_or_else(|| anyhow::anyhow!("Banner {} 不存在", banner_id.0))?;
            let banner = banner_lock.lock();
            if let Err(e) = validation::check_banner_coverage(
                &banner,
                &new_deck,
                &self.card_registry,
                &self.logic_registry
            ) {
                anyhow::bail!("Deck {} 更新后, Banner {} 校验失败: {}", id.0, banner_id.0, e)
            }
        }

        // 校验通过, 继续

        let new_path = self.build_file_path(&new_deck);
        Self::save_to_path(&new_deck, &new_path)?;

        self.deck_registry.remove(id);
        self.deck_registry.insert(new_deck.clone());
        self.deck_registry.insert_path(id, new_path.clone());    // 自动覆盖原有路径

        if let Some(old_path) = old_path_opt {
            if new_path != old_path && old_path.exists() {
                if let Err(e) = fs::remove_file(&old_path) {
                    tracing::warn!(
                        old_path = ?old_path,
                        error = %e,
                        "删除旧 Deck 文件失败, 需手动清理残留文件"
                    )
                }
            }
        } else {
            tracing::warn!("Deck {} 的旧文件路径未记录, 跳过删除", id.0);
        }

        Ok(new_deck)
    }

    pub fn delete_deck(&self, id: DeckId) -> Result<()> {
        let affected_banners = self.banner_registry.find_banners_by_deck(id);
        if !affected_banners.is_empty() {
            anyhow::bail!("Deck {} 被下列 Banner {:?} 引用, 无法删除", id.0, affected_banners);
        }

        if let Some(path) = self.deck_registry.get_path(id) {
            if path.exists() {
                fs::remove_file(&path)
                    .with_context(|| format!("删除 Deck 文件失败 - file: {:?}", path))?;
            }
        }

        self.deck_registry.remove(id);
        self.deck_registry.remove_path(id);

        Ok(())
    }

    pub fn build_file_path(&self, deck: &TaggedDeck) -> PathBuf {
        self.base_path.join(format!("{}.json", deck.inner.id.0))
    }

    fn save_to_path(deck: &TaggedDeck, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json_string = serde_json::to_string_pretty(deck)
            .with_context(|| "Deck 转为 JSON 文本时失败")?;

        let temp_path = path.with_extension(".tmp");
        fs::write(&temp_path, json_string)?;
        fs::rename(&temp_path, path)?;

        Ok(())
    }
}