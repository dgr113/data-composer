use std::fs;
use std::path::Path;
use std::hash::Hash;
use std::ffi::OsStr;
use std::borrow::Borrow;

use bson::Document;
use mongodb::sync::Collection;
use mongodb::options::DropCollectionOptions;

use data_finder::config::FinderConfig;
use data_getter::{ ResultParse, GetterConfig };

use crate::errors::ApiError;
use crate::config::ComposerConfig;
use crate::core::io_utils::{ dump_yaml, parse_yaml };
use crate::core::storage_utils::{ check_coll_exists, mongo_get_data, mongo_convert_results, prepare_to_doc };




pub struct ComposerIntro;

impl ComposerIntro {
    /** Get compose data from tree by natural key */
    pub fn get_from_tree<'a>(tree: &'a serde_yaml::Value, getter_config: &GetterConfig, access_key: &[String]) -> Result<serde_json::Value, ApiError> {
        let result = data_getter::run(tree, getter_config.clone(), access_key) ?;
        Ok( result )
    }

    /** Check state for update tree */
    pub fn get_tree<S, P>(finder_config: &FinderConfig, app_type: S, tree_path: P) -> Result<serde_yaml::Value, ApiError>
        where S: Into<String> + Hash + Eq, String: Borrow<S>,
              P: AsRef<Path> + AsRef<OsStr>
    {
        match Path::new( &tree_path ).exists() {
            true => fs::read_to_string( &tree_path ).map_err( |err| ApiError::IOError( err.to_string() ) ).and_then( parse_yaml ),  // If mapping tree file exists - return it
            false => ComposerBuild::get_updated_tree(finder_config, app_type, &tree_path)  // Else - build new mapping and return it
        }
    }

    /** Get full data from mapping tree
     * 'access_key' : Compose key for partial access to content Tree
     */
    pub fn get_full<S, K, P>(
        composer_config: &ComposerConfig,
        coll: &Collection,
        update: Option<bool>,
        filter: Option<&serde_json::Value>,
        id_key: Option<&str>,
        app_type: S,
        tree_path: P,
        access_key: &[K]
    )
        -> ResultParse<Vec<serde_json::Value>>
            where S: Into<String> + Hash + Eq, String: Borrow<S>,
                  K: Into<String> + Hash + Eq + serde_yaml::Index, String: Borrow<K>,
                  P: AsRef<Path> + AsRef<OsStr>
    {
        let filter = prepare_to_doc(filter, None).unwrap_or( Document::new() );
        if update.unwrap_or( false ) {
            let drop_coll_opts = DropCollectionOptions::default();
            coll.drop( drop_coll_opts ).unwrap();
        }
        if !check_coll_exists( coll ) {
            if ComposerBuild::get_updated_full(composer_config, coll, id_key, app_type, tree_path, access_key).is_err() {
                println!("Error with update assets data!")
            };
        }
        Ok( mongo_convert_results( mongo_get_data(coll, filter) ) )

        // coll.drop().unwrap();
        // ComposerBuild::get_updated_full(&params, finder_config, coll, id_key);
        // Ok( mongo_convert_results( mongo_get_data(coll, filter) ) )
    }
}



struct ComposerBuild;

impl ComposerBuild {
    /** Convert serde_json Value into vector or values (for later conversion in BSON docs) */
    fn prepare_external_data( v: &serde_json::Value ) -> Option<Vec<&serde_json::Value>> {
        let mut res = Vec::new();
        if v.is_object() {
            res = v.as_object().unwrap().values().collect();
        }
        else if v.is_array() {
            res = v.as_array().unwrap().iter().collect();
        }
        else {
            res.push( v );
        }
        Some( res )
    }

    /** Update data brief (sliced data by Id key from mapping tree) and return it
    *
    * # Parameters:
    * `update_mark_path`: Path to identify update process mark
    * `save_path`: Path to save result tree
    * `brief_fields`: Json fields for extracting
    * `add_key_components`: Additional external composite key components
    */
    fn get_updated_full<S, K, P>(
        composer_config: &ComposerConfig,
        coll: &Collection,
        id_key: Option<&str>,
        app_type: S,
        tree_path: P,
        access_key: &[K]
    )
        -> Result<serde_json::Value, ApiError>
            where S: Into<String> + Hash + Eq, String: Borrow<S>,
                  K: Into<String> + Hash + Eq + serde_yaml::Index, String: Borrow<K>,
                  P: AsRef<Path> + AsRef<OsStr>
    {
        let tree = Self::get_updated_tree(&composer_config.data_finder, app_type, tree_path).expect( "Error with create tree on full-update stage!" );

        let data_getter_result = data_getter::run(&tree, composer_config.data_getter.clone(), access_key) ?;

        Self::prepare_external_data( &data_getter_result )
            .ok_or(  ApiError::GetterApiError( "Error external data convert!".to_string() ) )
            .and_then( |arr| {
                let docs = arr.iter()
                    .map( |v| prepare_to_doc(Some(v), id_key) )
                    .filter( |d| d.is_some() )
                    .map( |d| d.unwrap().clone() )
                    .collect::<Vec<Document>>();

                let res = coll.insert_many(docs, None)
                    .map_err( |_| ApiError::GetterApiError( "Error write doc into Mongo!".to_string() ) );
                    // .and_then( |_| Ok( "Success write data" ) );
                res
            })
            .and_then( |_| Ok( data_getter_result ) )

    }

    /** Update mapping tree and return it
    *
    * # Parameters:
    * `update_mark_path`: Path to identify update process mark
    * `save_path`: Path to save result tree
    * `sniffer_config_path`: Path to sniffer for build tree
    * `app_type`: App type for access sniffer settings in config
    */
    fn get_updated_tree<S, P>(finder_config: &FinderConfig, app_type: S, tree_path: P) -> Result<serde_yaml::Value, ApiError>
        where S: Into<String> + Hash + Eq, String: Borrow<S>,
              P: AsRef<Path> + AsRef<OsStr>
    {
        let data_finder_result = data_finder::run(finder_config.clone(), app_type) ?;  // Run ext data-finder
        let content = serde_yaml::to_value( &data_finder_result ) ?;

        dump_yaml( content ).and_then( |content_str| {
            Path::new( &tree_path ).parent()
                .ok_or( ApiError::IOError( "Error with create <tree> directory!".to_string() ) )
                .and_then( |t| Ok( fs::create_dir_all( t ) ) )
                .and_then( |_| {
                    fs::write(tree_path, content_str)
                        .map_err( |_| ApiError::IOError( "Error with writing mapping file!".to_string() ) )
                        .map( |_| serde_yaml::Value::default() )
                }).expect( "Error with dump updated Tree!" );

            Ok( data_finder_result )
        })
    }
}
