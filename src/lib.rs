pub extern crate mongodb;
pub extern crate data_finder;
pub extern crate data_getter;

use std::io;

use mongodb::sync::Collection;
use serde_json::Value as SerdeJsonValue;

pub mod core;
pub use crate::core::config_utils::Params;
pub use crate::core::functions::ComposerIntro;

use data_getter::ResultParse;
use data_getter::GetterConfig;
use data_finder::config::FinderConfig;
use std::hash::Hash;
use std::borrow::Borrow;




pub struct ComposerApi;

impl ComposerApi {
    /** Get a brief description of a given content type
    *
    * # Parameters:
    * `id_key`: Field of every document in <arr_data> interpreted as database document ID
    */
    pub fn get_full<S>(
        finder_config: &FinderConfig,
        getter_config: &GetterConfig,
        app_type: S,
        coll: &Collection,
        access_key: &[S],
        update: Option<bool>,
        filter: Option<&SerdeJsonValue>,
        id_key: Option<&str>,
        tree_path: &str
    )
        -> ResultParse<Vec<SerdeJsonValue>>
            where S: Into<String> + ToString + Hash + Eq + serde_yaml::Index, String: Borrow<S>
    {
        // let access_key = &[lang, ];
        // let params = Params::build_params(getter_config, app_type, access_key);
        // ComposerIntro::get_full(params, finder_config, coll, update, filter, id_key)

        ComposerIntro::get_full(getter_config, finder_config, coll, update, filter, id_key, app_type, tree_path, access_key)
    }


    /** Get a brief description of a given content type */
    pub fn get_tree<S>(finder_config: &FinderConfig, app_type: S, tree_path: &str)
        -> Result<serde_yaml::Value, io::Error>
            where S: Into<String> + Hash + Eq, String: Borrow<S>
    {
        // let access_key = &["", ];
        // let params = Params::build_params(getter_config, app_type, access_key);
        ComposerIntro::get_tree(finder_config, app_type, tree_path)
    }
}
