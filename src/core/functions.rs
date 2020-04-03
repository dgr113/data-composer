use std::path::Path;
use std::{fs, io};
use std::collections::HashMap;

use mongodb::coll::Collection;
use data_getter::ResultParse;

use crate::core::common_utils::{get_dummy_error};
use crate::core::io_utils::{dump_json, parse_json, dump_yaml, parse_yaml};
use crate::core::storage_utils::{check_coll_exists, mongo_get_data, mongo_convert_results, prepare_to_doc};
pub use crate::core::config_utils::{TreeParams, BriefParams};
use bson::ordered::OrderedDocument;



pub struct ComposerIntro {}

impl ComposerIntro {

    /// Check state for update tree
    pub fn get_tree(tree_params: TreeParams) -> Result<serde_yaml::Value, io::Error> {
        match !Path::new(&tree_params.save_path).exists() {
            true => ComposerBuild::get_updated_tree(&tree_params),
            false => fs::read_to_string(tree_params.save_path).and_then(parse_yaml)
        }
    }


    pub fn get_full(coll: &Collection, tree_params: TreeParams, brief_params: BriefParams, update: Option<bool>, tree_order_key: &str, filter: Option<&serde_json::Value>, id_key: Option<&str>) -> ResultParse<Vec<serde_json::Value>> {
        let filter = prepare_to_doc(filter, None).unwrap_or(OrderedDocument::new());
        let is_force_update = update.unwrap_or(false);
        if is_force_update {
            coll.drop();
        }
        if is_force_update || !check_coll_exists(coll) {
            ComposerBuild::get_updated_full(coll, &tree_params, &brief_params, tree_order_key, id_key);
        }

        Ok( mongo_convert_results( mongo_get_data(coll, filter) ) )
    }


    /// Get compose (natural key for access YAML field)
    pub fn get_from_tree<'a>(tree: &'a serde_yaml::Value, access_key: &[&str]) -> ResultParse<serde_json::Value> {
        data_getter::run(tree, &access_key, "MESSAGE", None, None)
    }
}



struct ComposerBuild {}

impl ComposerBuild {
    /// Update brief (if needed)
    ///
    /// # Parameters:
    /// `update_mark_path`: Path to identifity update process mark
    /// `save_path`: Path to save result tree
    /// `brief_fields`: Json fields for extracting
    /// `add_key_components`: Additional external composite key components
    ///
    fn get_updated_full(coll: &Collection, tree_params: &TreeParams, brief_params: &BriefParams, tree_order_key: &str, id_key: Option<&str>) -> ResultParse<serde_json::Value> {
        let tree = Self::get_updated_tree(tree_params).expect("Error with create tree on full-update stage!");
        let brief_fields = &brief_params.brief_fields.iter().map(|s| s.as_str()).collect::<Vec<&str>>(); // NEED TO REFACTOR!

        data_getter::run(&tree, brief_params.access_key, "MESSAGE", Some(brief_fields), Some("."))
            .and_then(|results| {
                &results.as_array().ok_or("Error data getter")
                    .and_then(|arr| {
                        let docs = arr.iter()
                            .map(|v| prepare_to_doc(Some(v), Some("id")))
                            .filter(|d| d.is_some())
                            .map(|d| d.unwrap().clone())
                            .collect::<Vec<OrderedDocument>>();

                        Ok( coll.insert_many(docs, None).expect("Error write doc into Mongo!") )
                    });
                Ok( results )
            })
    }


    /// Update tree (if needed)
    ///
    /// # Parameters:
    /// `update_mark_path`: Path to identifity update process mark
    /// `save_path`: Path to save result tree
    /// `sniffer_config_path`: Path to sniffer for build tree
    /// `app_type`: App type for access sniffer settings in config
    ///
    fn get_updated_tree(params: &TreeParams) -> Result<serde_yaml::Value, io::Error> {
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
