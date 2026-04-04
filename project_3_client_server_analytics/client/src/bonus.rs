extern crate tarpc;

use std::time::Instant;
use std::io::BufRead;

use analytics_lib::{dataset::Value, query::{Aggregation, Condition, Query}};
use client::{start_client, solution};

use regex::Regex;  // We're gonna use the Regex library to make our lives easier
                   // And also because I love regex! - Leo Deng

// Your solution goes here.
fn parse_filter_from_string(filter_input: String) -> Condition {
    let filter_equal_string_pattern = Regex::new(r#"^(\w+) == "(\w+)"$"#).unwrap();
    let filter_equal_integer_pattern = Regex::new(r#"^(\w+) == (\d+)$"#).unwrap();
    let filter_not_pattern = Regex::new(r#"^!\((.*?)\)$"#).unwrap();
    let filter_and_pattern = Regex::new(r#"^\((.*?) AND (.*?)\)$"#).unwrap();
    let filter_or_pattern = Regex::new(r#"^\((.*?) OR (.*?)\)$"#).unwrap();

    match filter_equal_string_pattern.captures(&filter_input) {
        Some(filter_equal_string_result) => {
            let filter_equal_string_left = String::from(&filter_equal_string_result[1]);
            let filter_equal_string_right = String::from(&filter_equal_string_result[2]);
            return Condition::Equal(filter_equal_string_left, Value::String(filter_equal_string_right));
        }
        None => {}
    };

    match filter_equal_integer_pattern.captures(&filter_input) {
        Some(filter_equal_integer_result) => {
            let filter_equal_integer_left = String::from(&filter_equal_integer_result[1]);
            let filter_equal_integer_right = String::from(&filter_equal_integer_result[2]);
            return Condition::Equal(filter_equal_integer_left, Value::Integer(filter_equal_integer_right.parse().unwrap()));
        }
        None => {}
    };

    match filter_not_pattern.captures(&filter_input) {
        Some(filter_not_result) => {
            let filter_not_value = String::from(&filter_not_result[1]);
            return Condition::Not(Box::new(parse_filter_from_string(filter_not_value)));
        }
        None => {}
    };

    match filter_and_pattern.captures(&filter_input) {
        Some(filter_and_result) => {
            let filter_and_left = String::from(&filter_and_result[1]);
            let filter_and_right = String::from(&filter_and_result[2]);
            return Condition::And(Box::new(parse_filter_from_string(filter_and_left)), Box::new(parse_filter_from_string(filter_and_right)));
        }
        None => {}
    };

    match filter_or_pattern.captures(&filter_input) {
        Some(filter_or_result) => {
            let filter_or_left = String::from(&filter_or_result[1]);
            let filter_or_right = String::from(&filter_or_result[2]);
            return Condition::Or(Box::new(parse_filter_from_string(filter_or_left)), Box::new(parse_filter_from_string(filter_or_right)));
        }
        None => {}
    };

    panic!("Unable to parse the filter!");
}

fn parse_query_from_string(input: String) -> Query {    
    // Source: https://docs.rs/regex/latest/regex/ 
    let input_pattern = Regex::new(r#"^FILTER (.*?) GROUP BY (\w+) (COUNT|SUM|AVERAGE) (\w+)$"#).unwrap();
    let Some(input_result) = input_pattern.captures(&input) else {
        panic!("Unable to parse the input!");
    };

    let filter = parse_filter_from_string(String::from(&input_result[1]));
    let group_by = String::from(&input_result[2]);
    let aggregate = match &input_result[3] {
        "COUNT" => Aggregation::Count(String::from(&input_result[4])),
        "SUM" => Aggregation::Sum(String::from(&input_result[4])),
        "AVERAGE" => Aggregation::Average(String::from(&input_result[4])),
        _ => panic!("Unable to pasre aggregation!")
    };
    
    return Query::new(filter, group_by, aggregate);
}

// Each defined rpc generates an async fn that serves the RPC
#[tokio::main]
async fn main() {
    // Establish connection to server.
    let rpc_client = start_client().await;

    // Get a handle to the standard input stream
    let stdin = std::io::stdin();

    // Lock the handle to gain access to BufRead methods like lines()
    println!("Enter your query:");
    for line_result in stdin.lock().lines() {
        // Handle potential errors when reading a line
        match line_result {
            Ok(query) => {
                if query == "exit" {
                    break;
                }

                // parse query.
                let query = parse_query_from_string(query);

                // Carry out query.
                let time = Instant::now();
                let dataset = solution::run_fast_rpc(&rpc_client, query).await;
                let duration = time.elapsed();

                // Print results.
                println!("{}", dataset);
                println!("Query took {:?} to executed", duration);
                println!("Enter your next query (or enter exit to stop):");
            },
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
}