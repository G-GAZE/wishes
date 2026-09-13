use anyhow::Result;

use crate::{domain::{banner::TaggedBanner, deck::TaggedDeck, logic::definition::LogicVariant}, infrastructure::registry::{CardRegistry, LogicRegistry}};



pub fn check_banner_coverage(
    banner: &TaggedBanner,
    deck: &TaggedDeck,
    card_registry: &CardRegistry,
    logic_registry: &LogicRegistry,
) -> Result<()> {
    let logic_def = logic_registry
        .get_definition(banner.logic_instance.logic_id)
        .ok_or_else(|| anyhow::anyhow!("Logic 定义 {} 不存在", banner.logic_instance.logic_id.0))?;

    if let LogicVariant::Hardcoded { executor_name } = &logic_def.variant {
        let combos = logic_registry
            .hardcoded_possible_output_combinations(executor_name)
            .unwrap_or_default();

        let mut missing = Vec::new();
        for (tags, event_tags) in combos {
            let candidates = deck.query_cards(card_registry, &tags, &event_tags);
            if candidates.is_empty() {
                missing.push((tags, event_tags));
            }
        }

        if !missing.is_empty() {
            anyhow::bail!(
                "Deck {} 未能覆盖 Logic {} 的所有输出组合, 缺失组合: {:?}",
                deck.id.0,
                logic_def.id.0,
                missing
            )
        }
    }

    Ok(())
}