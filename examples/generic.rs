extern crate jsonpath_lib as jsonpath;
// use logicmaps::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
enum Comparison {
    Contains,
    IsIn,
    Equal,
    SumEqual,
    SumGreater,
    SumGreaterOrEqual,
}

#[derive(Debug, Deserialize)]
struct LogicBrick {
    name: String,
    jpath: String,
    value: Value,
    comparison: Comparison,
}

fn main() {
    let input = r#"
        {
            "name": "John Doe",
            "age": 43,
            "attibutes":
            {
                "fave_color": 942.2
            },
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ],
            "classes": [
            {
                "hours": 10,
                "sub": "SCI",
                "level": 100,
                "skill": [10,2,4]
            },
            {
                "hours": 25,
                "sub": "SCI",
                "level": 200,
                "skill": [10,2,4]
            },
            {
                "hours": 5,
                "sub": "SCI",
                "level": 500,
                "skill": [10,2,4]
            },
            {
                "hours": 50,
                "sub": "ENG",
                "level": 600,
                "skill": [10,2,4]
            }]
        }"#;

    // let data = r#"
    //     {
    //         "name": "FaveColorBlue",
    //         "jpath": "$.attibutes.fave_color",
    //         "value": 942.2
    //     }"#;
    // let data = r#"
    //     {
    //         "name": "OnePhoneNumberIs",
    //         "value": "+44 1234567",
    //         "comparison": "IsIn",
    //         "jpath": "$.phones"
    //     }"#;

    let data = r#"
        {
            "name": "AgeInRange",
            "value": [42,43,44,50,43],
            "comparison": "Contains",
            "jpath": "$.age"
        }"#;

    // let data = r#"
    //     {
    //         "name": "AtLeast40HoursOfScience",
    //         "value": 40,
    //         "comparison": "SumGreaterOrEqual",
    //         "jpath": "$.classes[?(@.sub == \"SCI\")]..hours"
    //     }"#;

    // let data = r#"
    //     {
    //         "name": "HasSkill",
    //         "value": 1,
    //         "comparison": "IsIn",
    //         "jpath": "$.classes..skill"
    //     }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(input).unwrap();
    // println!("{:?}", v);

    let logic_brick: LogicBrick = serde_json::from_str(data).unwrap();

    // println!("{:#?}", logic_brick);

    let mut selector = jsonpath::selector(&v);

    let path = logic_brick.jpath; //"$.attibutes.fave_color";
    let target = logic_brick.value; // "Blue";
    let json = selector(&path).unwrap();

    println!("{:#?}", json);

    match logic_brick.comparison {
        Comparison::Contains => {
            // // contains
            println!("{:?}", "contains");
            let mut indices: Vec<Option<usize>> = target
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
                .map(|(index, val)| {
                    if val == json[0] {
                        println!("json contains target {}", index);
                        return Some(index);
                    }
                    None
                })
                .collect();

            indices.retain(|c| c.is_some());
            for ix in indices {
                let hit_path = ix.unwrap(); //format!("{}[{}]", path, ix.unwrap());
                println!("{:?}", hit_path);
            }
        }
        Comparison::IsIn => {
            // // is_in
            let y = json[0].as_array();
            if y.unwrap().contains(&target) {
                println!("target is in json");
            };
        }
        Comparison::Equal => {
            // equal
            if json[0] == &target {
                println!("target is equal to json");
            };
        }
        Comparison::SumEqual => {
            // sum equal
            let y: Vec<f64> = json.iter().map(|x| x.as_f64().unwrap()).collect();
            let z: f64 = y.iter().sum();
            if z == target.as_f64().unwrap() {
                println!("target is equal to sum of json");
            };
        }
        Comparison::SumGreater => {
            // sum equal
            let y: Vec<f64> = json.iter().map(|x| x.as_f64().unwrap()).collect();
            let z: f64 = y.iter().sum();
            if z > target.as_f64().unwrap() {
                println!("target is greater to sum of json");
            };
        }
        Comparison::SumGreaterOrEqual => {
            // sum equal
            let y: Vec<f64> = json.iter().map(|x| x.as_f64().unwrap()).collect();
            let z: f64 = y.iter().sum();
            if z >= target.as_f64().unwrap() {
                println!("target is greater to sum of json");
            };
        }
    }

    // // greater_than
    // if json[0] > &target {
    //     println!("json is greater than target");
    // };
    // // less_than

    // if json[0] < &target {
    //     println!("json is less than target");
    // };

    // has_key

    // println!("{:#?}", v);
    // println!("{:#?}", z);
}
