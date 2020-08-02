use std::path::Path;
use std::{fs, io};

use mongodb::coll::Collection;
use data_getter::ResultParse;

use crate::core::common_utils::{get_dummy_error};
use crate::core::io_utils::{dump_yaml, parse_yaml};
use crate::core::storage_utils::{check_coll_exists, mongo_get_data, mongo_convert_results, prepare_to_doc};
pub use crate::core::config_utils::Params;
use bson::ordered::OrderedDocument;
use serde_json::Value;

use rand::{Rng};





pub struct ComposerIntro {}

impl ComposerIntro {

    /** Convert serde_json Value into vector or values (for later conversion in BSON docs) **/
    fn prepare_external_data(v: &serde_json::Value) -> Option<Vec<&serde_json::Value>> {
        let mut res = Vec::new();
        if v.is_object() { res = v.as_object().unwrap().values().collect(); }
        else if v.is_array() { res = v.as_array().unwrap().iter().collect(); }
        else { res.push(v); }
        Some(res)
    }


    /// Check state for update tree
    pub fn get_tree(params: Params, finder_config: &Value) -> Result<serde_yaml::Value, io::Error> {
        match !Path::new(&params.tree_path).exists() {
            true => ComposerBuild::get_updated_tree(&params, finder_config),
            false => fs::read_to_string(params.tree_path).and_then(parse_yaml)
        }
    }


    pub fn get_full(params: Params, finder_config: &serde_json::Value, coll: &Collection, update: Option<bool>, filter: Option<&serde_json::Value>, id_key: Option<&str>)
        -> ResultParse<Vec<serde_json::Value>>
        {
            let filter = prepare_to_doc(filter, None).unwrap_or(OrderedDocument::new());
            let is_force_update = update.unwrap_or(false);

            // if is_force_update { coll.drop().unwrap(); }
            // if is_force_update || !check_coll_exists(coll) {
            //     if ComposerBuild::get_updated_full(&params, finder_config, coll, id_key).is_err() {
            //         println!("Error with update assets data!")
            //     };
            // }



            coll.drop().unwrap();
            let result = ComposerBuild::get_updated_full(&params, finder_config, coll, id_key).unwrap();
            // let result = rand::thread_rng().gen_range(0, 1000);
            Ok( vec![ result ] )



            // Ok( mongo_convert_results( mongo_get_data(coll, filter) ) )
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
    fn get_updated_full(params: &Params, finder_config: &Value, coll: &Collection, id_key: Option<&str>) -> ResultParse<serde_json::Value> {
        let tree = Self::get_updated_tree(params, finder_config).expect("Error with create tree on full-update stage!");
        let brief_fields = &params.brief_fields.iter().map(|s| s.as_ref()).collect::<Vec<&str>>(); // NEED TO REFACTOR!

        data_getter::run(&tree, params.access_key, "MESSAGE", Some(brief_fields), Some("."))
            .and_then(|results| {
                Ok(results)

                // ComposerIntro::prepare_external_data(&results)
                //     .ok_or("Error external data convert!".to_string())
                //     .and_then(|arr| {
                //         let docs = arr.iter()
                //             .map(|v| prepare_to_doc(Some(v), id_key))
                //             .filter(|d| d.is_some())
                //             .map(|d| d.unwrap().clone())
                //             .collect::<Vec<OrderedDocument>>();
                //         coll.insert_many(docs, None)
                //             .map_err(|_| "Error write doc into Mongo!".to_string())
                //             .and_then(|_| Ok("Success write data"))
                //     })
                //     .and_then(|_| Ok(results))
            })
    }


    /// Update tree and return it
    ///
    /// # Parameters:
    /// `update_mark_path`: Path to identifity update process mark
    /// `save_path`: Path to save result tree
    /// `sniffer_config_path`: Path to sniffer for build tree
    /// `app_type`: App type for access sniffer settings in config
    ///
    fn get_updated_tree(params: &Params, finder_config: &Value) -> Result<serde_yaml::Value, io::Error> {
        let result = data_finder::run(finder_config.clone(), &params.app_type);  // Run ext data-finder

        serde_yaml::to_value(&result)
            .or_else(get_dummy_error)
            .and_then(dump_yaml)
            .and_then(|content| {
                Path::new(&params.tree_path).parent()
                    .ok_or("Error with create <tree> directory!")
                    .and_then(|t| Ok(fs::create_dir_all(t)))
                    .and_then(|_| fs::write( &params.tree_path, content ).map_err(|_| "Error with write <tree> file!"))
                    .unwrap();

                Ok(result)
            })
    }
}
