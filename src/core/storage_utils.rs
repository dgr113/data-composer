use std::fs;

use serde_json::Value;
use bson::ordered::OrderedDocument;
use mongodb::coll::Collection;
use mongodb::Client;

use crate::mongodb::ThreadedClient;
use crate::mongodb::db::ThreadedDatabase;




pub fn read_json(file_path: &str) -> serde_json::Value {
    fs::read_to_string(file_path).and_then(
        |content| Ok(serde_json::from_str(&content).expect("Error convert JSON string into Value!"))
    ).unwrap_or(Value::Null)
}


pub fn convert_to_doc(d: Option<&serde_json::Value>) -> OrderedDocument {
    match d {
        Some(t) => {
            let result_: bson::Bson = t.clone().into();  // Maybe need to optimize ...
            result_.as_document().expect("Error converting JSON Value into Bson filter!").clone()
        },
        None => OrderedDocument::new()
    }
}


pub fn mongo_get_coll(db_uri: &str, db_name: &str, coll_name: &str) -> Collection {
    let client = Client::with_uri(db_uri).expect("Error: Failed to initialize MongoDb client!");
    let db = client.db(db_name);
    db.collection(coll_name)
}


/**  Save docs into MongoDB and optional set ID (id needed and exists) **/
///
/// # Parameters:
/// `id_field`: Field of every document in <arr_data> interpreted as database document ID
///
pub fn mongo_save_data(coll: &Collection, arr_data: &[serde_json::Value], id_field: Option<&str>) {
    let docs: Vec<OrderedDocument> = arr_data.clone().iter()
        .map(|d| convert_to_doc(Some(d)))
        .map(|mut d| {
            if let Some(id_) = id_field {
                if d.contains_key(id_) {
                    d.insert("_id", d.get(id_).unwrap().clone());  // Maybe need to be optimize ...
                }
            }
            d
        })
        .collect();

    coll.insert_many(docs, None).expect("Error write doc into Mongo!");
}


/**  Get docs from MongoDB by filter (if needed) **/
pub fn mongo_get_data(coll: &Collection, filter: OrderedDocument) -> Vec<OrderedDocument> {
    match coll.find(Some(filter), None) {
        Ok(cursor) => cursor.map(|doc| doc.unwrap()).collect::<Vec<_>>(),
        Err(_err) => Vec::new()
    }
}


/** Convert MongoDB data results into serde_json Value **/
pub fn mongo_convert_results(results: Vec<OrderedDocument>) -> Vec<serde_json::Value> {
    let results = serde_json::to_string(&results).unwrap();
    let results: Vec<serde_json::Value> = serde_json::from_str(&results).unwrap();
    results
}


/** Checking whether the collection exists in this database **/
pub fn check_coll_exists(coll: &Collection) -> bool {
    match coll.find_one(Some(OrderedDocument::new()), None) {
        Ok(t) => t.is_some(),
        Err(_) => false
    }
}




/** ONLY FOR TEST USE **/
pub fn get_mongo_test(db_uri: &str, db_name: &str, db_coll: &str, data: &str, filter: Option<&serde_json::Value>, id_key: Option<&str>) -> Vec<serde_json::Value> {
    let coll = mongo_get_coll(db_uri, db_name, db_coll);
    let data: Value = serde_json::from_str(data).unwrap();

    match data["data"].as_array() {
        Some(data_arr) => {
            mongo_save_data(&coll, &data_arr, id_key);
            let filter = convert_to_doc(filter);
            let results = mongo_get_data(&coll, filter.clone());
            mongo_convert_results(results)
        },
        None => {
            println!("Error convert Jsn data into array!");
            vec![serde_json::Value::Null]
        }
    }
}
