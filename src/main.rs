use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tokio;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CoverageData {
    dsn: String,
    reporter: String,
    coverage: Value,
    projectID: String,
    commitSha: String,
    instrumentCwd: String,
}

async fn upload_coverage_data(data: &CoverageData) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .post(&data.dsn)
        .json(data)
        .header("Authorization", format!("Bearer {}", data.reporter))
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let response_json: Value = response.json().await?;
    println!("{:#?}", response_json);

    Ok(())
}

fn merge_coverage_map(first: &Value, second: &Value) -> Value {
    let mut first_map = first.as_object().unwrap().clone();
    let second_map = second.as_object().unwrap();

    for (k, v) in second_map.iter() {
        first_map.insert(k.clone(), v.clone());
    }

    Value::Object(first_map)
}

#[tokio::main]
async fn main() {
    let public_dir = ".canyon_output";
    let paths = fs::read_dir(public_dir).unwrap();

    let mut map: HashMap<String, CoverageData> = HashMap::new();

    let json_files: Vec<_> = paths
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "json" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    if json_files.is_empty() {
        println!("canyon: no coverage files found in .canyon_output");
    }

    for path in json_files {
        let json_data = fs::read_to_string(&path).unwrap();
        let data: Result<CoverageData, _> = serde_json::from_str(&json_data);
        println!("{:?}", data);
        if let Ok(data) = data {
            if let Some(existing_data) = map.get(&data.projectID) {
                let merged_coverage = merge_coverage_map(&existing_data.coverage, &data.coverage);
                map.insert(
                    data.projectID.clone(),
                    CoverageData {
                        coverage: merged_coverage,
                        ..data.clone()
                    },
                );
            } else {
                map.insert(data.projectID.clone(), data);
            }
        } else {
            println!("Invalid JSON format in file: {:?}", path);
        }
    }

    for value in map.values() {
        if let Err(e) = upload_coverage_data(value).await {
            eprintln!("Error uploading coverage data: {}", e);
        }
    }
}
