use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize)]
struct Record {
    edge1: String,
    edge2: String,
    weight: u16,
}

struct Graph {
    nodes: Vec<Node>
}

struct Node {
    name: String,
    edges: Vec<Edge>
}

struct Edge {
    source: String,
    destination: String,
    weight: u16,
}

fn get_data() -> Result<Vec<Record>, csv::Error> {

    let csv = "edge1,edge2,weight
    1,2,1
    1,3,1
    2,3,1
    2,5,1
    3,4,1
    4,5,1";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let mut vec: Vec<Record> = Vec::new();
    for record in reader.deserialize() {

        let record: Record = record?;
        vec.push(record);
    }

    Ok(vec)
}

fn main() -> Result<(), csv::Error> {

    let vec: Vec<Record> = get_data().unwrap();

    for record in vec {
        println!(
            "Edge {}, Edge 2 {}: Weight {}.",
            record.edge1,
            record.edge2,
            record.weight,
        );
    }
    map_data(vec);
    Ok(())
}

fn map_data(data: Vec<Record>) -> () {
    let mut g: HashMap<String, Node> = HashMap::new();
    for record in data {
        if g.contains_key(&record.edge1) {
            let e = Edge {
                source: record.edge1,
                destination: record.edge2,
                weight: record.weight,
            };
            g[&record.edge1].edges.push(e);
        } 
        else 
        {
            let e = Edge {
                source: record.edge1,
                destination: record.edge2,
                weight: record.weight,
            };
            let n = Node {
                name: record.edge1,
                edges: Vec::new(),
            };
            n.edges.push(e);
            g.insert(record.edge1, n);
        }

        for (key, edges) in g.iter() {
            println!("{}", edges);
            for edge in edges.iter() {
                println!("Destination {}, Weight {}", edge.destination, edge.weight)
            }
        }
    }

}
