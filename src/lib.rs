pub extern crate mongodb;
pub extern crate data_finder;
pub extern crate data_getter;

use std::ffi::OsStr;
use std::hash::Hash;
use std::path::Path;
use std::sync::RwLock;
use std::borrow::Borrow;

use mongodb::sync::Client;
use serde_json::Value as SerdeJsonValue;

pub mod core;
pub mod errors;
pub mod config;

use data_finder::config::FinderConfig;

pub use crate::errors::ApiError;
pub use crate::config::ComposerConfig;
pub use crate::core::functions::ComposerIntro;




pub struct ComposerApi;

impl ComposerApi {
    /** Get a brief description of a given content type
    *
    * # Parameters:
    * `id_key`: Field of every document in <arr_data> interpreted as database document ID
    */
    pub fn get_full<S, K, P>(
        composer_config: &ComposerConfig,
        app_type: S,
        db_pool: RwLock<Client>,
        access_key: &[K],
        update: Option<bool>,
        filter: Option<&SerdeJsonValue>,
        id_key: Option<&str>,
        tree_path: P
    )
        -> Result<Vec<SerdeJsonValue>, ApiError>
            where S: Into<String> + Hash + Eq, String: Borrow<S>,
                  K: Into<String> + Hash + Eq + serde_yaml::Index, String: Borrow<K>,
                  P: AsRef<Path> + AsRef<OsStr>
    {
        let app_type = app_type.into();
        let coll = db_pool.write().unwrap().database( &composer_config.database.db_name ).collection( &app_type );  // ПРОВЕРИТЬ ПУЛ СОЕДИНЕНИЯ С БД !
        let result = ComposerIntro::get_full(composer_config, &coll, update, filter, id_key, app_type, tree_path, access_key) ?;
        Ok( result )
    }


    /** Get a brief description of a given content type */
    pub fn get_tree<S, P>(finder_config: &FinderConfig, app_type: S, tree_path: P) -> Result<serde_yaml::Value, ApiError>
        where S: Into<String> + Hash + Eq, String: Borrow<S>,
              P: AsRef<Path> + AsRef<OsStr>
    {
        ComposerIntro::get_tree(finder_config, app_type, tree_path)
    }
}
