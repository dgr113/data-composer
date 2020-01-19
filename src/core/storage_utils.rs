/** For <mongodb> version 0.4.0 only **/

use std::fs;

use crate::mongodb::ThreadedClient;
use crate::mongodb::db::ThreadedDatabase;

use serde_json::Value;
use bson::ordered::OrderedDocument;
use mongodb::coll::Collection;
use mongodb::Client;



pub fn read_json(file_path: &str) -> serde_json::Value {
    fs::read_to_string(file_path).and_then(
        |content| Ok(serde_json::from_str(&content).expect("Error convert JSON string into Value!"))
    ).unwrap_or(Value::Null)
}


pub fn convert_to_doc(d: &serde_json::Value) -> OrderedDocument {
    let result: bson::Bson = d.clone().into();  // Maybe need to optimize ...
    result.as_document().expect("Error converting JSON Value into Bson filter!").clone()
}


pub fn mongo_get_coll(db_uri: &str, db_name: &str, coll_name: &str) -> Collection {
    let client = Client::with_uri(db_uri).expect("Error: Failed to initialize MongoDb client!");
    let db = client.db(db_name);
    db.collection(coll_name)
}


/** Save data into MongoDB **/
pub fn mongo_save_data(coll: &Collection, data: serde_json::Value, data_root_key: &str) {
    let docs: Vec<OrderedDocument> = data[data_root_key].as_array().unwrap().iter()
        .map(convert_to_doc).collect();

    coll.insert_many(docs, None).expect("Error write doc into Mongo!");
}


/** Get data from MongoDB **/
pub fn mongo_get_data(coll: &Collection, filter: OrderedDocument) -> Vec<OrderedDocument> {
    match coll.find(Some(filter), None) {
        Ok(cursor) => cursor.map(|doc| doc.unwrap()).collect::<Vec<_>>(),
        Err(_err) => Vec::new()
    }
}


/** Convert MongoDB data results into serde_json Value **/
pub fn mongo_convert_results(results: Vec<OrderedDocument>) -> serde_json::Value {
    let results = serde_json::to_string(&results).unwrap();
    let results: serde_json::Value = serde_json::from_str(&results).unwrap();
    results
}


/** Checking whether the collection exists in this database **/
pub fn check_coll_exists(coll: &Collection) -> bool {
    match coll.find_one(Some(OrderedDocument::new()), None) {
        Ok(t) => t.is_some(),
        Err(err) => false
    }
}



pub fn get_mongo_test(db_uri: &str, db_name: &str, db_coll: &str, data: &str) -> serde_json::Value {
    let mongo_coll = mongo_get_coll(db_uri, db_name, db_coll);

    let data: Value = serde_json::from_str(data).unwrap();

    mongo_save_data(&mongo_coll, data, "data");

    let filter_data: serde_json::Value = serde_json::from_str(r#"{"phones": {"$gte": 60}}"#).unwrap();
    let filter = convert_to_doc(&filter_data);

    let results = mongo_get_data(&mongo_coll, filter.clone());
    mongo_convert_results(results)
}
