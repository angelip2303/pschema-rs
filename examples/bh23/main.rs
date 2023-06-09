use std::time::Instant;

use pregel_rs::graph_frame::GraphFrame;
use pschema_rs::backends::ntriples::NTriples;
use pschema_rs::backends::Backend;
use pschema_rs::pschema::PSchema;
use pschema_rs::shape::shex::{
    Bound, Cardinality, NodeConstraint, ShapeAnd, ShapeOr, ShapeReference, TripleConstraint,
};

#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(target_env = "msvc")]
use mimalloc::MiMalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[cfg(target_env = "msvc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> Result<(), String> {
    // Define validation rules
    let shape = ShapeReference::new(
        "protein",
        "<http://purl.uniprot.org/core/annotation>",
        ShapeAnd::new(
            "annotation",
            vec![
                ShapeReference::new(
                    "reference_range",
                    "<http://purl.uniprot.org/core/range>",
                    ShapeAnd::new(
                        "grouping",
                        vec![
                            ShapeReference::new(
                                "range",
                                "<http://biohackathon.org/resource/faldo#begin>",
                                TripleConstraint::new(
                                    "position",
                                    "<http://biohackathon.org/resource/faldo#position>",
                                    NodeConstraint::Any,
                                )
                                .into(),
                            )
                            .into(),
                            ShapeReference::new(
                                "end",
                                "<http://biohackathon.org/resource/faldo#end>",
                                TripleConstraint::new(
                                    "position",
                                    "<http://biohackathon.org/resource/faldo#position>",
                                    NodeConstraint::Any,
                                )
                                .into(),
                            )
                            .into(),
                        ],
                    )
                    .into(),
                )
                .into(),
                TripleConstraint::new(
                    "comment",
                    "<http://www.w3.org/2000/01/rdf-schema#comment>",
                    NodeConstraint::Any,
                )
                .into(),
                Cardinality::new(
                    "cardinality",
                    ShapeOr::new(
                        "type",
                        vec![
                            TripleConstraint::new(
                                "Transmembrane_Annotation",
                                "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>",
                                NodeConstraint::Value(
                                    "<http://purl.uniprot.org/core/Transmembrane_Annotation>",
                                ),
                            )
                            .into(),
                            TripleConstraint::new(
                                "Transmembrane_Annotation",
                                "<http://www.w3.org/1999/02/22-rdf-syntax-ns#type>",
                                NodeConstraint::Value(
                                    "<http://purl.uniprot.org/core/Topological_Domain_Annotation>",
                                ),
                            )
                            .into(),
                        ],
                    )
                    .into(),
                    Bound::Zero,
                    Bound::Many,
                )
                .into(),
            ],
        )
        .into(),
    )
    .into();

    // Load Wikidata entities
    let edges = NTriples::import("uniprotkb_reviewed_viruses_10239_0.nt")?;

    // Perform schema validation
    match GraphFrame::from_edges(edges) {
        Ok(graph) => {
            let start = Instant::now();
            match PSchema::new(shape).validate(graph) {
                Ok(mut subset) => {
                    let duration = start.elapsed();
                    println!("Time elapsed in validate() is: {:?}", duration);
                    NTriples::export("uniprotkb_reviewed_viruses_10239_0-subset.nt", &mut subset)
                }
                Err(error) => Err(error.to_string()),
            }
        }
        Err(error) => Err(format!("Cannot create a GraphFrame: {}", error)),
    }
}
