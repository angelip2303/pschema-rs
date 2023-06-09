use pregel_rs::graph_frame::GraphFrame;
use pschema_rs::backends::duckdb::DuckDB;
use pschema_rs::backends::parquet::Parquet;
use pschema_rs::backends::Backend;
use pschema_rs::pschema::PSchema;
use pschema_rs::shape::shex::NodeConstraint;
use pschema_rs::shape::shex::Shape;
use pschema_rs::shape::shex::TripleConstraint;
use wikidata_rs::id::Id;

fn main() -> Result<(), String> {
    // Define validation rules
    let start = Shape::TripleConstraint(TripleConstraint::new(
        "City",
        u32::from(Id::from("P31")),
        NodeConstraint::Value(u32::from(Id::from("Q515"))),
    ));

    // Load Wikidata entities
    let edges = DuckDB::import("./examples/from_duckdb/3000lines.duckdb")?;

    // Perform schema validation
    match GraphFrame::from_edges(edges) {
        Ok(graph) => match PSchema::new(start).validate(graph) {
            Ok(mut result) => Parquet::export("3000lines-subset.parquet", &mut result),
            Err(error) => Err(error.to_string()),
        },
        Err(error) => Err(format!("Cannot create a GraphFrame: {}", error)),
    }
}
