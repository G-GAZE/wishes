//! # 卡片管理器

use std::{path::PathBuf, sync::Arc, fs};
use anyhow::{Context, Result};
use crate::{domain::{card::{Card, TaggedCard}, ids::CardId, tag::Tag}, infrastructure::registry::CardRegistry};


/// 卡片管理器.
/// 
/// 管理卡片的创建、查询、更新、删除, 同时管理卡片文件.
pub struct CardManager {
    /// `CardRegistry` 引用.
    registry: Arc<CardRegistry>,
    /// 卡片存储根目录, 应为 `data/cards`
    base_path: PathBuf,             // data/cards 目录
}

impl CardManager {
    pub fn new(registry: Arc<CardRegistry>, base_path: PathBuf) -> Self {
        Self {
            registry,
            base_path,
        }
    }

    /// 获取所有卡片的 `Arc` 引用.
    pub fn list_all(&self) -> Vec<Arc<TaggedCard>> {
        self.registry.all_cards()
    }

    /// 通过标签获取筛选后的卡片的 `Arc` 引用.
    pub fn list_by_tags(&self, tags: &[Tag]) -> Vec<Arc<TaggedCard>> {
        let ids = self.registry.tag_index.query(tags);
        ids.into_iter()
            .filter_map(|id| self.registry.get(id))
            .collect()
    }
    
    /// 创建一个新的卡片, 写入配置文件, 并返回为 `TaggedCard`.
    /// 
    /// # 参数
    /// - `content`: 对应 `Card.content`
    /// - `tags`: 卡片拥有的标签
    /// 
    /// # 失败
    /// 若写入文件失败, 则不会创建卡片.
    pub fn create_card(&self, content: String, tags: Vec<Tag>) -> Result<TaggedCard> {
        // 分配 Id
        let id = self.registry.allocate_id();

        // 创建 Card
        let card = Card {
            id,
            content,
        };
        let tagged_card = TaggedCard::with_tags(card, tags.into_iter().collect());

        // 写入文件
        let file_path = self.build_file_path(&tagged_card);
        Self::save_to_path(&tagged_card, &file_path)?;

        // 若写入成功, 更新内存
        self.registry.insert(tagged_card.clone());
        self.registry.insert_path(id, file_path);

        Ok(tagged_card)
    }

    /// 更新指定 Id 的卡片信息, 返回新的卡片为 `TaggedCard`.
    /// 
    /// # 参数
    /// - `id`: 要更新的卡片 Id
    /// - `new_content`: 卡片新的内容, 对应 `Card.content`
    /// - `new_tags`: 卡片更新后拥有的标签
    /// 
    /// # 失败
    /// - 指定 Id 的卡片不存在
    /// - 卡片的存储文件不存在
    /// - 写入文件失败
    /// 
    /// 任何失败, 都不会成功创建卡片.
    /// 
    /// 若删除旧文件失败, 则仅记录警告.
    pub fn update_card(&self, id: CardId, new_content: String, new_tags: Vec<Tag>) -> Result<TaggedCard> {
        // 获取旧卡片信息
        let ord_card = self.registry.get(id)
            .ok_or_else(|| anyhow::anyhow!("Card {} 不存在", id.0))?;
        let old_path = self.registry.get_path(id)
            .ok_or_else(|| anyhow::anyhow!("Card {} 的存储文件不存在", id.0))?;

        // 构建新卡片
        let mut new_card = ord_card.as_ref().clone();
        new_card.inner.content = new_content;
        new_card.tags = new_tags.into_iter().collect();

        let new_path = self.build_file_path(&new_card);

        // 尝试写入新文件
        Self::save_to_path(&new_card, &new_path)?;

        // 更新内存
        self.registry.remove(id);
        self.registry.insert(new_card.clone());
        self.registry.insert_path(id, new_path.clone());    // DashMap::insert 会自动覆盖原有旧路径

        // 删除旧文件
        if new_path != old_path {
            if old_path.exists() {
                if let Err(e) = fs::remove_file(&old_path) {
                    tracing::warn!(
                        old_path = ?old_path,
                        error = %e,
                        "删除旧 Card 文件失败, 需手动清理残留文件"
                    );
                }
            }
        }

        Ok(new_card)
    }

    /// 删除指定 Id 的卡片.
    /// 
    /// # 参数
    /// - `id`: 要删除卡片的 Id
    /// 
    /// 若删除卡片文件失败, 则不会删除卡片并返回错误.
    pub fn delete_card(&self, id: CardId) -> Result<()> {
        // TODO: 检查 Card 是否被 Deck 引用

        if let Some(path) = self.registry.get_path(id) {
            if path.exists() {
                fs::remove_file(&path)
                    .with_context(|| format!("删除 Card 文件失败 - file: {:?}", path))?;
            }
        }

        self.registry.remove(id);
        self.registry.remove_path(id);

        Ok(())
    }

    /// 根据卡片标签构建卡片存储路径.
    /// 
    /// 路径格式: `base_path/{game}/{type}/{rarity}/{id}.json`
    /// 缺失的标签值使用 `default` 代替.
    fn build_file_path(&self, card: &TaggedCard) -> PathBuf {
        let game = card
            .get_tag(Tag::NAMESPACE_GAME)
            .map(|t| t.value)
            .unwrap_or_else(|| "default".to_string());

        let type_ = card
            .get_tag(Tag::NAMESPACE_TYPE)
            .map(|t| t.value)
            .unwrap_or_else(|| "default".to_string());

        let rarity = card
            .get_tag(Tag::NAMESPACE_RARITY)
            .map(|t| t.value)
            .unwrap_or_else(|| "default".to_string());

        let filename = format!("{}.json", card.inner.id.0);

        self.base_path
            .join(game)
            .join(type_)
            .join(rarity)
            .join(filename)
    }

    /// 原子写入卡片配置文件.
    fn save_to_path(card: &TaggedCard, path: &PathBuf) -> Result<()> {
        // 确保目录存在
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json_string = serde_json::to_string_pretty(card)
            .with_context(|| "Card 转为 JSON 文本时失败")?;

        // 原子写入, 写入临时文件后重命名替换
        let temp_path = path.with_extension(".tmp");
        fs::write(&temp_path, json_string)?;
        fs::rename(&temp_path, path)?;

        Ok(())
    }
}