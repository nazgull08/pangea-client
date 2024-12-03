mod schemas;

use assert_json_diff::{assert_json_matches_no_panic, CompareMode, Config};
use serde_json::Value;
use std::{collections::HashMap, process::Command};

async fn run_example(data_source: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let output = Command::new("cargo")
        .args(["run", "--example", data_source])
        .output()
        .expect("Failed to execute example");

    if !output.status.success() {
        return Err(format!(
            "Error running example {data_source}: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let logs: Vec<Value> = stdout
        .lines()
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();

    Ok(logs)
}

#[tokio::test]
async fn examples_json_schema_tests() -> Result<(), Box<dyn std::error::Error>> {
    let schema_map: HashMap<&str, fn() -> Value> = HashMap::from([
        (
            "jsonstream-status",
            schemas::jsonstream_status::schema as fn() -> Value,
        ),
        (
            "jsonstream-blocks",
            schemas::jsonstream_blocks::schema as fn() -> Value,
        ),
    ]);

    for (data_source, schema_fn) in &schema_map {
        println!("Checking \x1b[1m{data_source}\x1b[0m schema...");

        let logs = run_example(data_source).await?;
        let schema = schema_fn();

        let config = Config::new(CompareMode::Strict);
        for log in logs {
            if let Err(err) = assert_json_matches_no_panic(&log, &schema, config.clone()) {
                return Err(format!("Schema mismatch for {data_source}: {err}").into());
            }
        }
    }

    Ok(())
}
