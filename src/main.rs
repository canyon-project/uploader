use clap::{Parser, Subcommand};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use chrono;
use serde_json::Value;
use uploader::merge::{merge_coverage_map};

#[derive(Parser)]
#[command(name = "canyon-uploader")]
#[command(about = "一个用于上传覆盖率数据的工具")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 输出版本信息
    Version,
    /// 扫描目录并上传数据
    Map {
        /// 指定要扫描的目录路径
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CoverageData {
    dsn: String,
    reporter: String,
    coverage: Value,
    projectID: String,
    commitSha: String,
    instrumentCwd: String,
//     这里也要写全了！！！
    branch: Option<String>,
    compareTarget: Option<String>,
}

// 表示单个文件的覆盖率信息
// 这里要写全了！！！
#[derive(Serialize, Deserialize, Debug, Clone)]
struct FileCoverage {
    path: String,
    s: Value,
    f: Value,
    b: Value,
    branchMap: Option<Value>,
    fnMap: Option<Value>,
    statementMap: Option<Value>,
    inputSourceMap: Option<Value>,
    provider: Option<Value>,
    instrumentCwd: String,
    version: Option<Value>,
    projectID: String,
    sha: String,
    branch: Option<String>,
    dsn: Option<String>,
    reporter: Option<String>,
    compareTarget: Option<String>,
}

// 表示所有文件的覆盖率集合，键为文件路径，值为单个文件的覆盖率信息
type CoverageCollection = std::collections::BTreeMap<String, FileCoverage>;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Some(Commands::Version) => {
            println!("canyon-uploader 版本 1.0.0");
        }
        Some(Commands::Map { path }) => {
            // 外部传入path
            let path = path.unwrap_or_else(|| std::env::current_dir().unwrap());
            let public_dir = path.join(".canyon_output");

            // 检查目录是否存在
            if !public_dir.exists() {
                log("info", &format!("Directory '{}' not found", public_dir.display()));
                return;
            }

            // 读取目录下的所有文件
            let paths = fs::read_dir(public_dir).unwrap();

            // 创建一个HashMap
            let mut map: HashMap<String, CoverageData> = HashMap::new();

            // json文件需要形如 coverage-final-xxx.json
            let json_files: Vec<_> = paths
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let path = entry.path();
                    let file_name = path.file_name()?.to_str()?;
                    if file_name.starts_with("coverage-final-") && file_name.ends_with(".json") {
                        Some(path)
                    } else {
                        None
                    }
                })
                .collect();

            // 没有找到json文件
            if json_files.is_empty() {
                log("info", &format!("No coverage files found in .canyon_output"));
            }

            // 遍历json文件
            for path in json_files {
                let json_data = fs::read_to_string(&path).unwrap();

                let data: Result<CoverageCollection, _> = serde_json::from_str(&json_data);

                if let Ok(data) = data {
                    // 打印data的第一个key
                    if let Some((key, _)) = data.iter().next() {
                        log("info", &format!("key is {:?}", key));
                    }

                    // 从 data 中取第一个值来获取公共信息
                    if let Some((_, first_value)) = data.iter().next() {
                        let coverage = serde_json::to_value(data.clone()).unwrap();
                        let data = CoverageData {
                            coverage,
                            projectID: first_value.projectID.clone(),
                            commitSha: first_value.sha.clone(),
                            instrumentCwd: first_value.instrumentCwd.clone(),
                            dsn: first_value.dsn.clone().unwrap_or_else(|| "".to_string()),
                            reporter: first_value.reporter.clone().unwrap_or_else(|| "".to_string()),
                            branch: first_value.branch.clone(),
                            compareTarget: first_value.compareTarget.clone(),
                        };

                        if let Some(existing_data) = map.get(&data.projectID) {
                            println!("Merging coverage data for project: {}", data.projectID);
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
                    } else {
                        log("error", &format!("No valid data in file: {:?}", path));
                    }
                } else {
                    log("error", &format!("Failed to parse JSON in file: {:?}", path));
                }
            }

            // 打印map
            log("info", &format!("Merged map: {:?}", map));
            for value in map.values() {
                if let Err(e) = upload_coverage_data(value).await {
                    log("error", &format!("Error uploading coverage data: {}", e));
                }
            }
        }
        None => {
            eprintln!("请指定一个子命令");
        }
    }
}

async fn upload_coverage_data(data: &CoverageData) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // 把请求体积写到本地
    fs::write("request_body.json", serde_json::to_string_pretty(data).unwrap()).unwrap();

    // let response = client
    //     .post(&data.dsn)
    //     .json(data)
    //     .header("Authorization", format!("Bearer {}", data.reporter))
    //     .header("Content-Type", "application/json")
    //     .send()
    //     .await?;

    log("info", &format!("Uploading data: {:?}", data));
    // let response_json: serde_json::Value = response.json().await?;
    log("info", &"Upload successful!".to_string());
    Ok(())
}

fn log(level: &str, message: &str) {
    let start = SystemTime::now();
    let datetime: chrono::DateTime<chrono::Utc> = start.into();
    let timestamp = datetime.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    println!("[{}] ['{}'] => {}", timestamp, level, message);
}