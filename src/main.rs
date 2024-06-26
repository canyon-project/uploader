// mod merge;

// use merge::coverage_merge::{merge_coverage_map, FileCoverage};


use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use std::collections::HashMap;
use std::fs;
use tokio;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono;
use uploader::merge::{merge_coverage_map};


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
    log("info", &format!("Upload successful!"));

    Ok(())
}

// fn merge_coverage_map(first: &Value, second: &Value) -> Value {
//     let mut first_map = first.as_object().unwrap().clone();
//     let second_map = second.as_object().unwrap();
//
//     for (k, v) in second_map.iter() {
//         first_map.insert(k.clone(), v.clone());
//     }
//
//     Value::Object(first_map)
// }
// merge_coverage_map()
// merge
pub fn generate_header(version: &str) -> String {
    format!(
        r#"
   ____
  / ___|__ _ _ __  _   _  ___  _ __
 | |   / _\` | '_ \\| | | |/ _ \\| '_ \\
 | |__| (_| | | | | |_| | (_) | | | |
  \\____\\__,_|_| |_|\\__, |\\___/|_| |_|
                   |___/

  Canyon report uploader {}
"#, version)
}

fn log(level: &str, message: &str) {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let datetime: chrono::DateTime<chrono::Utc> = start.into();
    let timestamp = datetime.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    println!("[{}] ['{}'] => {}", timestamp, level, message);
}
#[tokio::main]
async fn main() {
    let version = "0.0.1";
    let header = generate_header(version);
    println!("{}", header);
    let public_dir = ".canyon_output";
    if !Path::new(public_dir).exists() {
        log("info", &format!("Directory '{}' not found", public_dir));
        return;
    }
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
        log("info", &format!("No coverage files found in .canyon_output"));
    }
    // merge.
    for path in json_files {
        let json_data = fs::read_to_string(&path).unwrap();
        let data: Result<CoverageData, _> = serde_json::from_str(&json_data);
        // println!("{:?}", data);???
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
                map.insert(data.projectID.clone(), data.clone());
            }
            log("info", &format!("{:?} merge success! projectID is {:?}, sha is {:?}, reportID is {:?}", path ,data.projectID, data.commitSha, data.commitSha));
        } else {
            log("info", &format!("Invalid JSON format in file: {:?}", path));
        }
    }

    for value in map.values() {
        if let Err(e) = upload_coverage_data(value).await {
            log("error", &format!("Error uploading coverage data: {}", e));
        }
    }
}
