#![feature(convert)]
#[cfg(test)]
#[warn(unused_imports)]

use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::string::String;
use std::thread::sleep_ms;
use std::collections::BTreeMap;
use RethinkDB;
use api::*;
use core::*;



struct Person {
    name : String,
    age  : i32
}

// // socat  -v -x TCP4-LISTEN:7888,fork,reuseaddr TCP4:localhost:28015
// #[test]
// fn test_create() {
//     let mut rethinkdb = RethinkDB::connect("localhost", 7888, "", 3);
//     let db = db("test");
//     let tc = db.table_create("person_create").replicas(1i32).run(&mut rethinkdb);
//     let td = db.table_drop("person_create").run(&mut rethinkdb);
//     assert_eq!(1, 2);

// }

// #[test]
// fn test_insert() {
//     let mut rethinkdb = RethinkDB::connect("localhost", 7888, "", 3);
// 	let db = db("test");
//     db.table_create("person").replicas(1i32).run(&mut rethinkdb);

//     let mut nachoData = BTreeMap::new();
//     nachoData.insert("name".to_string(), Json::String("Nacho".to_string()));
//     nachoData.insert("age".to_string(), Json::I64(6i64));

//     let tc = db.table("person").insert(nachoData).run(&mut rethinkdb);
//     let td = db.table_drop("person").run(&mut rethinkdb);

// }

// #[test]
// fn test_insert_option_conflict_update() {//TODO get last inserted and try to update it
//     let mut rethinkdb = RethinkDB::connect("localhost", 7888, "", 3);
//     let mut nachoData = BTreeMap::new();
//     nachoData.insert("name".to_string(), Json::String("Nacho".to_string()));
//     nachoData.insert("age".to_string(), Json::I64(8i64));
//     let db = db("test");
//     let tc = db.table("person").insert(nachoData).conflict("update").run(&mut rethinkdb);

//     assert_eq!(1,2);
// }

#[test]
fn test_get() {
    let mut rethinkdb = RethinkDB::connect("localhost", 7888, "", 3);
    let db = db("test");

    //let tc = db.table_create("person_get").primary_key("name".to_string()).run(&mut rethinkdb);

    let mut nachoData = BTreeMap::new();
    nachoData.insert("name".to_string(), Json::String("Nacho".to_string()));
    nachoData.insert("age".to_string(), Json::I64(6i64));

    //db.table("person_get").insert(nachoData).run(&mut rethinkdb);

    let nacho_json = db.table("person_get").get(Json::String("Nacho".to_string())).run(&mut rethinkdb);
    println!("{:?}", nacho_json);
    match nacho_json.find_path(&["r", "name"]).unwrap() {
        &Json::String(ref name) => assert_eq!(*name, "Nacho".to_string()),
        _ => panic!("The returned object is strange")
    }

/*=======
    let tc = db.table_create("person_get").primary_key("name".to_string()).run(&mut rethinkdb);
    sleep_ms(5000);

    let mut nacho_data = BTreeMap::new();
    nacho_data.insert("name".to_string(), Json::String("Nacho".to_string()));
    nacho_data.insert("age".to_string(), Json::I64(6i64));
>>>>>>> created write module*/

    /*#[derive(RustcDecodable, RustcEncodable)]
    struct Person  {
        name: String,
        age: i64
    }

    let nachoData = Person{
        name: "Nacho".to_string(),
        age: 6
    };

        let encoded = json::encode(&nacho_data).unwrap();
    */


    db.table("person_get").insert(Json::Object(nachoData)).run(&mut rethinkdb);
    db.table("person_get").get(Json::String("Nacho".to_string())).run(&mut rethinkdb);


}
