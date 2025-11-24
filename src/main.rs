use clap::Parser;
use reqwest::Error as ReqwestError;
use serde_json::Value;

#[derive(Parser)]
#[command(version,
    after_help = "Examples:\n\nList the slug of all Bug Bounty Programs in Immunefi\n-> ibb\n\nAll details about Moonbeam Network's Bug Bounty Program\n-> ibb moonbeamnetwork\n\nOnly assets in scope and their metadata\n-> ibb moonbeamnetwork assets\n\nOnly URLs\n-> ibb moonbeamnetwork assets url\n\nThe protocol documentation 0x listed in their Bug Bounty Program\n-> ibb 0x programDocumentations\n\nFind recursively any field returned by Immunefi's REST API for a specific Bug Bounty Program\n\n-> ibb [any_program] [any_field]\n\nFilter the output as much as desired by adding nested fields\n-> ibb [any_program] [any_field] [nestedfield_1] [nestedfield_2]\n-> ibb moonbeamnetwork bounty impacts title",
    about = "Is like jq for Immunefi's REST API. Search, filter and map structured data about bug bounty programs with ease.",
    long_about = None,
    author = "infosec_us_team"
)]
struct Cli {
    /// [protocol_name] [field] [another_field] ...
    query: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), ReqwestError> {
    // Parse the CLI command
    let cli = Cli::parse();

    // Check if there are any arguments in the query
    if cli.query.is_empty() {
        // If no query, print the name of all bug bounty programs

        // Define the API endpoint for bug bounty programs
        const API_ENDPOINT: &str = "https://cdn.jsdelivr.net/gh/infosec-us-team/Immunefi-Bug-Bounty-Programs-Unofficial/projects.json";

        // Send a GET request to the API endpoint
        let response = reqwest::get(API_ENDPOINT).await?;

        // Ensure the request was successful
        if response.status().is_success() {
            // Parse JSON response
            let json: Value = response.json().await?;

            // Extract and print all bug bounty IDs
            let mut ids = Vec::new();
            projects_ids(&json, &mut ids);
            println!("{:?}", ids);
        } else {
            // Print error if request was unsuccessful
            eprintln!("Error: {}", response.status());
        }
    } else {
        // If query has arguments, retrieve specific bug bounty program info

        // Extract the first argument as the bug bounty program ID
        let bbp = cli.query.first().cloned().unwrap_or_default();

        // Construct API endpoint URL for specific bug bounty program
        let api_endpoint = "https://cdn.jsdelivr.net/gh/infosec-us-team/Immunefi-Bug-Bounty-Programs-Unofficial/project/".to_string() + &bbp + ".json";

        // Send a GET request to the constructed endpoint
        let response = reqwest::get(api_endpoint).await?;

        // Ensure the request was successful
        if response.status().is_success() {
            // Parse JSON response
            let json: Value = response.json().await?;

            // Check if there are additional arguments
            if cli.query.len() > 1 {
                // If more arguments, find specific information about the program
                let result = find_values_by_keys(&json, &cli.query[1..]);
                println!("{}", serde_json::to_string_pretty(&result).unwrap())
            } else {
                // If no additional arguments, print entire bug bounty program info
                match serde_json::to_string(&json) {
                    Ok(json_string) => println!("{}", json_string), // Successfully print JSON
                    Err(e) => eprintln!("Error serializing JSON: {}", e), // Handle serialization error
                }
            }
        } else {
            // Print error if request was unsuccessful
            eprintln!("Error: {}", response.status());
        }
    }
    Ok(())
}

/// Recursively searches for all "slug" fields in a JSON structure, concatenates each "slug" value into a single string, and appends a newline after each found "slug".
fn projects_ids(value: &Value, ids: &mut Vec<String>) {
    match value {
        // If the value is an object, process it as a map
        Value::Object(map) => {
            // Check if there's an "slug" field, and if so, add it to ids
            if let Some(id) = map.get("slug").and_then(Value::as_str) {
                ids.push(id.to_string());
            } else {
                // If no "slug" field, recursively search through each value in the map
                for v in map.values() {
                    projects_ids(v, ids);
                }
            }
        }

        // If the value is an array, recursively search through each element
        Value::Array(arr) => {
            arr.iter().for_each(|v| projects_ids(v, ids));
        }

        // Ignore other types (no action needed)
        _ => {}
    }
}

/// Recursively searches for values in a JSON structure based on a vector of keys.
fn find_values_by_keys(value: &Value, keys: &[String]) -> Value {
    // Return Null if no keys are provided
    if keys.is_empty() {
        return Value::Null;
    }

    // Define the first key and the remaining keys
    let first_key = &keys[0];
    let remaining_keys = &keys[1..];

    match value {
        Value::Object(map) => {
            let mut results = Vec::new();

            // Iterate over each key-value pair in the object
            for (key, sub_value) in map {
                // Check if the current key matches the target key
                if key == first_key {
                    if remaining_keys.is_empty() {
                        // If the value is an array, return it directly
                        if sub_value.is_array() {
                            return sub_value.clone();
                        } else {
                            results.push(sub_value.clone());
                        }
                    } else {
                        // Recursively search deeper with remaining keys
                        let found_value = find_values_by_keys(sub_value, remaining_keys);
                        if found_value != Value::Null && found_value.get(0).is_some() {
                            if let Value::Array(found_array) = found_value {
                                results.extend(found_array); // Flatten arrays
                            }
                        }
                    }
                } else {
                    // Recursively search all other paths for current keys
                    let found_value = find_values_by_keys(sub_value, keys);
                    if found_value != Value::Null && found_value.get(0).is_some() {
                        if let Value::Array(found_array) = found_value {
                            results.extend(found_array); // Flatten arrays
                        }
                    }
                }
            }

            // Return results as an array if any found, else Null
            if results.is_empty() {
                Value::Null
            } else {
                Value::Array(results)
            }
        }

        Value::Array(arr) => {
            let mut results = Vec::new();

            // Iterate over each element in the array and search for keys
            for item in arr {
                let found_value = find_values_by_keys(item, keys);
                if found_value != Value::Null && found_value.get(0).is_some() {
                    if let Value::Array(found_array) = found_value {
                        results.extend(found_array);
                    }
                }
            }

            // Return results as an array if any found, else Null
            if results.is_empty() {
                Value::Null
            } else {
                Value::Array(results)
            }
        }

        _ => Value::Null, // Return Null if the value type isn't Object or Array
    }
}
