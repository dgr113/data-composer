pub extern crate mongodb;
pub extern crate data_finder;
pub extern crate data_getter;

use std::io;
use std::hash::Hash;
use std::path::Path;
use std::ffi::OsStr;
use std::borrow::Borrow;

use mongodb::sync::Collection;
use serde_json::Value as SerdeJsonValue;

pub mod core;
pub use crate::core::config_utils::Params;
pub use crate::core::functions::ComposerIntro;

use data_getter::ResultParse;
use data_getter::GetterConfig;
use data_finder::config::FinderConfig;




pub struct ComposerApi;

impl ComposerApi {
    /** Get a brief description of a given content type
    *
    * # Parameters:
    * `id_key`: Field of every document in <arr_data> interpreted as database document ID
    */
    pub fn get_full<S, K, P>(
        finder_config: &FinderConfig,
        getter_config: &GetterConfig,
        app_type: S,
        coll: &Collection,
        access_key: &[K],
        update: Option<bool>,
        filter: Option<&SerdeJsonValue>,
        id_key: Option<&str>,
        tree_path: P
    )
        -> ResultParse<Vec<SerdeJsonValue>>
        where S: Into<String> + Hash + Eq, String: Borrow<S>,
              K: Into<String> + Hash + Eq + serde_yaml::Index, String: Borrow<K>,
              P: AsRef<Path> + AsRef<OsStr>
    {
        // let access_key = &[lang, ];
        // let params = Params::build_params(getter_config, app_type, access_key);
        // ComposerIntro::get_full(params, finder_config, coll, update, filter, id_key)
        ComposerIntro::get_full(getter_config, finder_config, coll, update, filter, id_key, app_type, tree_path, access_key)
    }


    /** Get a brief description of a given content type */
    pub fn get_tree<S, P>(finder_config: &FinderConfig, app_type: S, tree_path: P)
        -> Result<serde_yaml::Value, io::Error>
            where S: Into<String> + Hash + Eq, String: Borrow<S>,
                  P: AsRef<Path> + AsRef<OsStr>
    {
        // let access_key = &["", ];
        // let params = Params::build_params(getter_config, app_type, access_key);
        ComposerIntro::get_tree(finder_config, app_type, tree_path)
    }
}
