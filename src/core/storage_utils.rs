/** For <mongodb> version 0.4.0 only **/

use std::fs;

use crate::mongodb::ThreadedClient;
use crate::mongodb::db::ThreadedDatabase;

use serde_json::Value;
use bson::ordered::OrderedDocument;
use mongodb::coll::Collection;
use mongodb::Client;



fn read_json(file_path: &str) -> serde_json::Value {
    fs::read_to_string(file_path).and_then(
        |content| Ok(serde_json::from_str(&content).expect("Error convert JSON string into Value!"))
    ).unwrap_or(Value::Null)
}


fn convert_to_doc(d: &serde_json::Value) -> OrderedDocument {
    let result: bson::Bson = d.clone().into();  // Maybe need to optimize ...
    result.as_document().expect("Error converting JSON Value into Bson filter!").clone()
}


fn mongo_get_coll(db_host: &str, db_port: u16, db_name: &str, coll_name: &str) -> Collection {
    let client = Client::connect(db_host, db_port).expect("Error: Failed to initialize MongoDb client!");
    let db = client.db(db_name);
    db.collection(coll_name)
}


fn mongo_save(coll: &Collection, data: serde_json::Value, data_root_key: &str) {
    let docs: Vec<OrderedDocument> = data[data_root_key].as_array().unwrap().iter()
        .map(convert_to_doc).collect();

    coll.insert_many(docs, None).expect("Error write doc into Mongo!");
}


fn mongo_get(coll: &Collection, filter: OrderedDocument) -> Vec<OrderedDocument> {
    match coll.find(Some(filter), None) {
        Ok(cursor) => cursor.map(|doc| doc.unwrap()).collect::<Vec<_>>(),
        Err(_err) => Vec::new()
    }
}


fn mongo_convert_test(results: Vec<OrderedDocument>) -> serde_json::Value {
    let results = serde_json::to_string(&results).unwrap();
    let results: serde_json::Value = serde_json::from_str(&results).unwrap();
    results
}




pub fn get_mongo_test(db_host: &str, db_port: u16) -> serde_json::Value {
    let mongo_db_name = "test_db";
    let mongo_db_coll = "test_coll";
    let mongo_coll = mongo_get_coll(db_host, db_port, mongo_db_name, mongo_db_coll);

//    let data = read_json("test.json");
    let data = r#"
        {
          "data": [
            {
              "name": "John Doe",
              "age": 43,
              "phones": [
                10,
                50
              ]
            },
            {
              "name": "Augistene Vene",
              "age": 15,
              "phones": [
                60,
                70
              ]
            }
          ]
        }"#;
    let data: Value = serde_json::from_str(data).unwrap();

    mongo_save(&mongo_coll, data, "data");

    let filter_value: serde_json::Value = serde_json::from_str(r#"{"phones": {"$gte": 60}}"#).unwrap();
    let filter: bson::Bson = filter_value.into();
    let filter = filter.as_document().expect("Error converting JSON Value into Bson filter!");

    let results = mongo_get(&mongo_coll, filter.clone());
    mongo_convert_test(results)
}
