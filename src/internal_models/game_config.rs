use std::sync::Arc;
use crate::external_models::base_level::BaseLevel;
use crate::external_models::path_config::PathConfig;

pub struct GameConfig {
    pub base_levels: Arc<Vec<BaseLevel>>, // all available base levels
    pub paths: Arc<PathConfig>,           // settings containing paths between bases
}