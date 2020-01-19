use std::path::Path;
use std::{fs, io};
use std::collections::HashMap;

use data_getter::ResultParse;
use crate::core::io_utils::{dump_json, parse_json, dump_yaml, parse_yaml};
use crate::core::common_utils::{get_dummy_error};
pub use crate::core::config_utils::{TreeParams, BriefParams};
use crate::core::storage_utils::{mongo_get_coll, check_coll_exists, mongo_get_data, convert_to_doc, mongo_convert_results};


pub struct ExtraInterface {}

impl ExtraInterface {

    /// Check state for update tree
    pub fn get_tree(tree_params: TreeParams) -> Result<serde_yaml::Value, io::Error> {
        match !Path::new(&tree_params.save_path).exists() {
            true => IntroInterface::update_tree(&tree_params),
            false => fs::read_to_string(tree_params.save_path).and_then(parse_yaml)
        }
    }


    pub fn get_full(tree_params: TreeParams, brief_params: BriefParams, tree_order_key: &str, filter: serde_json::Value) -> ResultParse<serde_json::Value> {
        let coll = mongo_get_coll(&brief_params.tmp_db_uri, &brief_params.tmp_db_name, &brief_params.app_type);
        let filter = convert_to_doc(&filter);

        match check_coll_exists(&coll) {
            false => IntroInterface::update_full(&tree_params, &brief_params, tree_order_key),
            true => {
                let results = mongo_get_data(&coll, filter);
                mongo_convert_results(results)
            }
        }
    }


    /// Get compose (natural key for access YAML field)
    pub fn get_from_tree<'a>(tree: &'a serde_yaml::Value, access_key: &[&str]) -> ResultParse<serde_json::Value> {
        data_getter::run(tree, &access_key, "MESSAGE", None, None)
    }
}



struct IntroInterface {}

impl IntroInterface {
    /// Get main ordered keys from file or from tree
    fn get_ordered_keys(tree: serde_yaml::Value, brief_params: &BriefParams, tree_order_key: &str) -> Vec<String> {
       match fs::read_to_string(&brief_params.order_path) {
            Ok(content) => content.lines().map(String::from).collect::<Vec<_>>(),
            Err(_) => {
                println!("Warning: Assets order file not found! Getting keys from Tree...");
                let root_mapping: HashMap<String, serde_yaml::Value> = serde_yaml::from_value(tree).expect("Error parse Content Tree mapping (content-machiner)");
                root_mapping[tree_order_key].as_mapping().expect("Error parse Content Tree mapping (content-machiner)")
                    .iter().map(|(k, _)| k.as_str().unwrap().to_string()).collect::<Vec<_>>()
            }
        }
    }


    /// Update brief (if needed)
    ///
    /// # Parameters:
    /// `update_mark_path`: Path to identifity update process mark
    /// `save_path`: Path to save result tree
    /// `brief_fields`: Json fields for extracting
    /// `add_key_components`: Additional external composite key components
    ///
    fn update_full(tree_params: &TreeParams, brief_params: &BriefParams, tree_order_key: &str) -> ResultParse<serde_json::Value> {
        let tree = Self::update_tree(tree_params).expect("Error with create tree on full-update stage!");
        let brief_fields = &brief_params.brief_fields.iter().map(|s| s.as_str()).collect::<Vec<&str>>(); // NEED TO REFACTOR!

        data_getter::run(&tree, brief_params.access_key, "MESSAGE", Some(brief_fields), Some("."))
            .and_then(|result|
                serde_json::to_value(&result)
                    .or_else(get_dummy_error)
                    .and_then(dump_json)
                    .and_then(|v| {
                        fs::create_dir_all(Path::new(&brief_params.save_path).parent().unwrap());  // Try to create parent directories
                        fs::write(&brief_params.save_path, v).or_else(|err| {
                            println!("Error write briefs into file: {}", &err.to_string());
                            get_dummy_error(err)
                        });
                        Ok(result)
                    }).map_err(|err| err.to_string())
            )
    }


    /// Update tree (if needed)
    ///
    /// # Parameters:
    /// `update_mark_path`: Path to identifity update process mark
    /// `save_path`: Path to save result tree
    /// `sniffer_config_path`: Path to sniffer for build tree
    /// `app_type`: App type for access sniffer settings in config
    ///
    fn update_tree(params: &TreeParams) -> Result<serde_yaml::Value, io::Error> {
        let result = data_finder::run(params.sniffer_config_path, params.app_type);  // Run ext sniffer

        serde_yaml::to_value(&result)
            .or_else(get_dummy_error)
            .and_then(dump_yaml)
            .and_then(|content| {
                fs::create_dir_all(Path::new(&params.save_path).parent().unwrap());  // Try to create parent directories
                fs::write(&params.save_path, content).or_else(|err|{
                    println!("Error write trees into file: {}", &err.to_string());
                    get_dummy_error(err)
                });
                Ok(result)
        })
    }
}
