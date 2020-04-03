use std::fs;

use serde_json::Value;
use bson::ordered::OrderedDocument;
use mongodb::coll::Collection;



pub fn read_json(file_path: &str) -> serde_json::Value {
    fs::read_to_string(file_path).and_then(
        |content| Ok(serde_json::from_str(&content).expect("Error convert JSON string into Value!"))
    ).unwrap_or(Value::Null)
}


/** Prepare one element to doc **/
pub fn prepare_to_doc(d: Option<&serde_json::Value>, id_field: Option<&str>) -> Option<OrderedDocument> {
    d.and_then(|t| {
        if t.is_object() {
            Some( t.to_owned().into() )
        }
        else if t.is_array() {
            Some( t[0].to_owned().into() )
        }
        else {
            None
        }
    }).and_then(|t: bson::Bson| {
        t.as_document().and_then(|d| {
            let mut d = d.clone();
            if let Some(id) = id_field {
                if d.contains_key(id) {
                    d.insert("_id", d.get(id).unwrap().clone());  // Maybe need to be optimize ...
                }
            }
            Some( d )
        })
    })
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
