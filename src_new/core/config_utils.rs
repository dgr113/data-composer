use std::collections::HashMap;
use crate::core::io_utils::{build_path, build_filename};



pub struct TreeParams<'a> {
    pub save_path: String,
    pub sniffer_config_path: &'a str,
    pub app_type: &'a str
}

impl<'a> TreeParams<'a> {
    pub fn build_params(config: &'a HashMap<String, String>, app_type: &'a str) -> TreeParams<'a> {
        let sniffer_config_path = &config["SNIFFER_CONFIG_PATH"];
        let tree_path = build_path(&config["TREES_BASEDIR"], &build_filename(&app_type,None, "yml", "."));
        TreeParams {
            save_path: tree_path.to_string(),
            sniffer_config_path,
            app_type
        }
    }
}



pub struct BriefParams<'a> {
    pub app_type: String,
    pub tree_path: String,
    pub tmp_db_uri: String,
    pub tmp_db_name: String,
    pub order_path: String,
    pub brief_fields: Vec<String>,
    pub access_key: &'a [&'a str]
}

impl<'a> BriefParams<'a> {
    pub fn build_params(config: &'a HashMap<String, String>, app_type: &'a str, access_key: &'a [&'a str]) -> BriefParams<'a> {
        let tree_path = build_path(&config["TREES_BASEDIR"], &build_filename(&app_type,None, "yml", "."));
        let order_path = build_path(&config["ORDERS_BASEDIR"], &build_filename(&app_type, None, "order", "."));
        let tmp_db_uri = &config["TMP_DB_URI"];
        let tmp_db_name = &config["TMP_DB_NAME"];

        let brief_fields = &config["BRIEFLY_FIELDS"].as_str().split(",").map(|s| s.trim().to_string()).collect::<Vec<String>>();
        BriefParams {
            tree_path,
            order_path,
            access_key,
            app_type: app_type.to_string(),
            tmp_db_uri: tmp_db_uri.clone(),
            tmp_db_name: tmp_db_name.clone(),
            brief_fields: brief_fields.to_vec(),
        }
    }
}
