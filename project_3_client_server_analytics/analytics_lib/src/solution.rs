use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

pub fn filter_parse(dataset: &Dataset, row: &Row, filter: &Condition) -> bool {
    // Maps filter and parses them by operations
    match filter {
        Condition::Equal(column_name, expected_value) => filter_equal(dataset, row, column_name, expected_value),
        Condition::Not(subfilter) => filter_not(dataset, row, subfilter),
        Condition::And(subfilter_left, subfilter_right) => filter_and(dataset, row, subfilter_left, subfilter_right),
        Condition::Or(subfilter_left, subfilter_right) => filter_or(dataset, row, subfilter_left, subfilter_right)
    }
}

pub fn filter_equal(dataset: &Dataset, row: &Row, column_name: &String, expected_value: &Value) -> bool {
    // Gets row value by fetching the column index from the dataset
    let column_index = dataset.column_index(&column_name);
    let row_value = row.get_value(column_index);

    // Compares row value and expected value
    *row_value == *expected_value
}

pub fn filter_not(dataset: &Dataset, row: &Row, filter: &Condition) -> bool {
    // Parses condition into boolean, and then performs "not" on it
    !filter_parse(dataset, row, filter)
}

pub fn filter_and(dataset: &Dataset, row: &Row, filter_left: &Condition, filter_right: &Condition) -> bool {
    // Parses both conditions into booleans, and then performs "and" on them
    filter_parse(dataset, row, filter_left) && filter_parse(dataset, row, filter_right)
}

pub fn filter_or(dataset: &Dataset, row: &Row, filter_left: &Condition, filter_right: &Condition) -> bool {
    // Parses both conditions into booleans, and then performs "or" on them
    filter_parse(dataset, row, filter_left) || filter_parse(dataset, row, filter_right)
}

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    // Creates blank dataset (dataset columns are the same since we're only filtering)
    let columns = dataset.columns();
    let mut filtered_dataset = Dataset::new(columns.clone());

    // Checks every row and appends to new dataset if passses
    for row in dataset.iter() {
        if filter_parse(dataset, row, filter) {
            filtered_dataset.add_row(row.clone());
        }
    }

    // Returns new dataset
    return filtered_dataset;
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    // Creates empty hashmap
    let mut grouped_hashmap = HashMap::new();

    // Gets the column index that we are grouping by
    let column_index = dataset.column_index(group_by_column);

    // Gets the column template (dataset columns are the same since we are not modifying the shape of the dataset)
    let columns = dataset.columns();

    // Checks every row and appends to corresponding dataset based on its value under the column
    for row in dataset.iter() {
        // Gets the row value
        let row_value = row.get_value(column_index);
        
        // Creates new dataset if the group does not exist
        if !grouped_hashmap.contains_key(row_value) {
            grouped_hashmap.insert(row_value.clone(), Dataset::new(columns.clone()));
        }

        // Appends row to the corresponding dataset
        let grouped_dataset = grouped_hashmap.get_mut(row_value).unwrap();
        grouped_dataset.add_row(row.clone());
    }

    // Returns new hashmap
    return grouped_hashmap;
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    todo!("Implement this!");
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}