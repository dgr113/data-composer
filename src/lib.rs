extern crate bson;
extern crate mongodb;
extern crate serde_json;
extern crate serde_yaml;
extern crate data_finder;
extern crate data_getter;

use std::io;
use std::collections::HashMap;

pub mod core;
pub use crate::core::functions::{ExtraInterface};
pub use crate::core::config_utils::{TreeParams, BriefParams};
pub use crate::core::storage_utils::get_mongo_test;
use data_getter::ResultParse;
use bson::ordered::OrderedDocument;



pub struct ApiInterface {}

impl ApiInterface {
    /// Get a brief description of a given content type
    pub fn get_full(app_type: &str, lang: &str, config: HashMap<String, String>, filter: Option<&serde_json::Value>) -> ResultParse<serde_json::Value> {
        let access_key = &[lang, ];
        let tree_params = TreeParams::build_params(&config, app_type);
        let brief_params = BriefParams::build_params(&config, app_type, access_key);
        ExtraInterface::get_full(tree_params, brief_params, "mapping", filter)
    }


    /// Get a brief description of a given content type
    pub fn get_tree(app_type: &str, config: HashMap<String, String>) -> Result<serde_yaml::Value, io::Error> {
        let params = TreeParams::build_params(&config, app_type);
        ExtraInterface::get_tree(params)
    }


    /// ONLY FOR TEST !
    pub fn get_storage_test(db_uri: &str, db_name: &str, db_coll: &str, data: &str) -> serde_json::Value {
        get_mongo_test(db_uri, db_name, db_coll, data)
    }
}
