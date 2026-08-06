use crate::domain::ids::LogicId;


#[derive(Debug, thiserror::Error)]
pub enum LogicError {
    #[error("Logic definition not found: {0:?}")]
    DefinitionNotFound(LogicId),
    #[error("Logic executor not found: {0}")]
    ExecutorNotFound(String),
    #[error("Rule executor not found: {0}")]
    RuleNotFound(String),
    #[error("Custom rule not found: {0}")]
    CustomRuleNotFound(String),
    #[error("Custom rule is not usable now")]
    CustomRuleUnusable(),
}