//! Output formatting utilities

use crate::cli::OutputFormat;
use anyhow::Result;
use serde::Serialize;
use tabled::{settings::Style, Table};

/// Format output according to the specified format
pub fn format_output<T>(data: &T, format: &OutputFormat) -> Result<()>
where
    T: Serialize,
{
    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(data)?;
            println!("{}", json);
        }
        OutputFormat::Yaml => {
            // For now, use JSON format as yaml isn't in dependencies
            let json = serde_json::to_string_pretty(data)?;
            println!("{}", json);
        }
        OutputFormat::Table => {
            // Convert to JSON value for table formatting
            let json_value = serde_json::to_value(data)?;
            print_table_from_json(&json_value);
        }
    }
    Ok(())
}

/// Print a table from JSON value
fn print_table_from_json(value: &serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            let mut rows = Vec::new();
            for (key, val) in map {
                let value_str = match val {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Null => "null".to_string(),
                    _ => serde_json::to_string(val).unwrap_or_else(|_| "".to_string()),
                };
                rows.push([key.clone(), value_str]);
            }

            if !rows.is_empty() {
                let table = Table::new(rows).with(Style::rounded()).to_string();
                println!("{}", table);
            }
        }
        serde_json::Value::Array(arr) => {
            // For arrays, print each item
            for item in arr {
                print_table_from_json(item);
                println!();
            }
        }
        _ => {
            // For simple values, just print them
            println!(
                "{}",
                serde_json::to_string_pretty(value).unwrap_or_else(|_| "".to_string())
            );
        }
    }
}
