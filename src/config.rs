use serde::{ Serialize, Deserialize };

use data_getter::GetterConfig;
use data_finder::config::FinderConfig;




#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComposerConfig {
    pub filters: Vec<serde_json::Value>,
    pub data_finder: FinderConfig,
    pub data_getter: GetterConfig,
    pub trees_basedir: String,
    pub orders_basedir: String
}
