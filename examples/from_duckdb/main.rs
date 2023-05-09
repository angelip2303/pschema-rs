use crate::duckdb_dump::DumpUtils;
use crate::pschema::PSchema;
use crate::rules::Rule;

use ego_tree::tree;
use polars::prelude::*;
use pregel_rs::graph_frame::GraphFrame;
use pschema_rs::duckdb_dump::DumpUtils;
use pschema_rs::pschema::PSchema;

fn main() -> Result<(), String> {
    // Define validation rules
    let tree = tree! {
        Rule::new("A", 31, 31, 1000000571) => {
            Rule::new("B", 31, 31, 1000000571) => {
                Rule::new("C", 31, 31, 1000000571),
                Rule::new("D", 31, 31, 1000000571),
            },
            Rule::new("E", 31, 31, 1000000571) => {
                Rule::new("F", 31, 31, 1000000571),
                Rule::new("G", 31, 31, 1000000571),
            },
            Rule::new("H", 31, 31, 1000000571),
        }
    };

    // Load Wikidata entities
    let edges = DumpUtils::edges_from_duckdb("./examples/pschema/example.duckdb")?;

    // Perform schema validation
    match GraphFrame::from_edges(edges) {
        Ok(graph) => match PSchema::new(tree).validate(graph) {
            Ok(result) => {
                println!("Schema validation result:");
                println!(
                    "{:?}",
                    result
                        .lazy()
                        .select(&[col("id"), col("labels")])
                        .filter(col("labels").is_not_null())
                        .collect()
                );
                Ok(())
            }
            Err(error) => Err(error.to_string()),
        },
        Err(_) => Err(String::from("Cannot create a GraphFrame")),
    }
}
