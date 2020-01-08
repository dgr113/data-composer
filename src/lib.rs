extern crate serde_json;
extern crate serde_yaml;
extern crate data_finder;
extern crate data_getter;

use std::io;
use std::collections::HashMap;

pub mod core;
pub use crate::core::functions::{ExtraInterface};
pub use crate::core::config_utils::{TreeParams, BriefParams};
use data_getter::ResultParse;



pub struct ApiInterface {}

impl ApiInterface {
    /// Get a brief description of a given content type
    pub fn get_full(app_type: &str, lang: &str, config: HashMap<String, String>, force_update: bool) -> ResultParse<serde_json::Value> {
        let access_key = &[lang, ];
        let tree_params = TreeParams::build_params(&config, app_type);
        let brief_params = BriefParams::build_params(&config, app_type,  access_key);
        ExtraInterface::get_full(tree_params, brief_params, "mapping", force_update)
    }


    /// Get a brief description of a given content type
    pub fn get_tree(app_type: &str, config: HashMap<String, String>, force_update: bool) -> Result<serde_yaml::Value, io::Error> {
        let params = TreeParams::build_params(&config, app_type);
        ExtraInterface::get_tree(params, force_update)
    }
}
