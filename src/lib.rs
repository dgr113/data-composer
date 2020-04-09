extern crate bson;
extern crate mongodb;
extern crate serde_json;
extern crate serde_yaml;
extern crate data_finder;
extern crate data_getter;

use std::io;

pub mod core;
pub use crate::core::functions::{ComposerIntro};
pub use crate::core::config_utils::{Params};
use data_getter::ResultParse;
use mongodb::coll::Collection;



pub struct ComposerApi {}

impl ComposerApi {
    /// Get a brief description of a given content type
    ///
    /// # Parameters:
    /// `id_key`: Field of every document in <arr_data> interpreted as database document ID
    ///
    pub fn get_full(coll: &Collection, app_type: &str, lang: &str, update: Option<bool>, config: &serde_json::Value, filter: Option<&serde_json::Value>, id_key: Option<&str>)
        -> ResultParse<Vec<serde_json::Value>>
        {
            let access_key = &[lang, ];
            let params = Params::build_params(config, app_type, access_key);
            ComposerIntro::get_full(coll, params, update, filter, id_key)
        }


    /// Get a brief description of a given content type
    pub fn get_tree(app_type: &str, config: &serde_json::Value) -> Result<serde_yaml::Value, io::Error> {
        let params = Params::build_params(config, app_type, &["", ]);
        ComposerIntro::get_tree(params)
    }
}
