use serde::{Serialize,Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]

struct Person {
   name:String,
   age:i32
}

fn main() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();

    println!("{:#?}", point);


    println!("{:?}", serialized);

    let de_serialized: Point = serde_json::from_str(&serialized).unwrap();


    let test = "{\"name\":\"Sam\",\"age\":22}";
    let de_serialized: Person = serde_json::from_str(&test).unwrap();

    println!("{:?}", de_serialized);




}