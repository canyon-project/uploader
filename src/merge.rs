use serde_json::{Value, Map, Number};

#[derive(Clone, Debug)]
pub struct FileCoverage {
    pub s: Map<String, Value>,
    pub f: Map<String, Value>,
    pub b: Map<String, Value>,
}

pub fn merge_file_coverage(first: &Value, second: &Value) -> Value {
    let mut ret = first.clone();

    let first_s = first.get("s").unwrap().as_object().unwrap().clone();
    let second_s = second.get("s").unwrap().as_object().unwrap();
    let first_f = first.get("f").unwrap().as_object().unwrap().clone();
    let second_f = second.get("f").unwrap().as_object().unwrap();
    let first_b = first.get("b").unwrap().as_object().unwrap().clone();
    let second_b = second.get("b").unwrap().as_object().unwrap();

    let mut merged_s = first_s.clone();
    for (k, v) in second_s {
        let count = merged_s.entry(k.clone()).or_insert(Value::Number(Number::from(0)));
        *count = Value::Number(Number::from(count.as_u64().unwrap() + v.as_u64().unwrap()));
    }

    let mut merged_f = first_f.clone();
    for (k, v) in second_f {
        let count = merged_f.entry(k.clone()).or_insert(Value::Number(Number::from(0)));
        *count = Value::Number(Number::from(count.as_u64().unwrap() + v.as_u64().unwrap()));
    }

    let mut merged_b = first_b.clone();
    for (k, v) in second_b {
        let ret_array = merged_b.entry(k.clone()).or_insert(Value::Array(vec![Value::Number(Number::from(0)); v.as_array().unwrap().len()]));
        let ret_array = ret_array.as_array_mut().unwrap();
        let second_array = v.as_array().unwrap();
        for (i, val) in second_array.iter().enumerate() {
            ret_array[i] = Value::Number(Number::from(ret_array[i].as_u64().unwrap() + val.as_u64().unwrap()));
        }
    }

    let mut result = Map::new();
    result.insert("s".to_string(), Value::Object(merged_s));
    result.insert("f".to_string(), Value::Object(merged_f));
    result.insert("b".to_string(), Value::Object(merged_b));
    Value::Object(result)
}

pub fn merge_coverage_map(first: &Value, second: &Value) -> Value {
    let mut first_map = first.as_object().unwrap().clone();
    let second_map = second.as_object().unwrap();

    for (k, v) in second_map {
        if !first_map.contains_key(k) {
            first_map.insert(k.clone(), v.clone());
        } else {
            let merged_coverage = merge_file_coverage(first_map.get(k).unwrap(), v);
            first_map.insert(k.clone(), merged_coverage);
        }
    }

    Value::Object(first_map)
}