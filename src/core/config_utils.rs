use serde_json;
use crate::core::io_utils::{build_path, build_filename};



pub struct Params<'a> {
    pub data_finder_config: &'a serde_json::Value,
    pub app_type: String,
    pub tree_path: String,
    pub order_path: String,
    pub brief_fields: Vec<String>,
    pub access_key: &'a [&'a str]
}

impl<'a> Params<'a> {
    pub fn build_params(config: &'a serde_json::Value, app_type: &'a str, access_key: &'a [&'a str]) -> Params<'a> {
        println!("COMPOSER CONFIG: {:?}", &config);
        let tree_path = build_path(
            &config["data-getter"]["TREES_BASEDIR"].as_str().unwrap(),
            &build_filename(&app_type,None, "yml", ".")
        );
        let order_path = build_path(
            &config["data-getter"]["ORDERS_BASEDIR"].as_str().unwrap(),
            &build_filename(&app_type, None, "order", ".")
        );

        let brief_fields = &config["data-getter"]["BRIEFLY_FIELDS"].as_str().unwrap()
            .split(",")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        let data_finder_config = &config["data-finder"];
        Params {
            data_finder_config,
            tree_path,
            order_path,
            access_key,
            app_type: app_type.to_string(),
            brief_fields: brief_fields.to_vec(),
        }
    }
}
