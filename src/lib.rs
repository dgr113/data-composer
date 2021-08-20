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




pub struct ComposerApi;

impl ComposerApi {
    /** Get a brief description of a given content type
    *
    * # Parameters:
    * `id_key`: Field of every document in <arr_data> interpreted as database document ID
    */
    pub fn get_full(
        finder_config: &SerdeJsonValue,
        getter_config: &SerdeJsonValue,
        app_type: &str,
        coll: &Collection,
        lang: &str,
        update: Option<bool>,
        filter: Option<&SerdeJsonValue>,
        id_key: Option<&str>

    )  -> ResultParse<Vec<SerdeJsonValue>> {

        let access_key = &[lang, ];
        let params = Params::build_params(getter_config, app_type, access_key);
        ComposerIntro::get_full(params, finder_config, coll, update, filter, id_key)
    }

    /** Get a brief description of a given content type */
    pub fn get_tree(finder_config: &SerdeJsonValue, getter_config: &SerdeJsonValue, app_type: &str) -> Result<serde_yaml::Value, io::Error> {
        let access_key = &["", ];
        let params = Params::build_params(getter_config, app_type, access_key);
        ComposerIntro::get_tree(params, finder_config)
    }
}
