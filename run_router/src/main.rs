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
    1,4,1
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

    for record in &vec {
        println!(
            "Edges: {},{}: Weight: {}.",
            record.edge1,
            record.edge2,
            record.weight,
        );
    }
    let g = map_data(&vec);

    for (key, node) in g.iter()
    {
        for edge in node.edges.iter() {
            println!("Key: {}, Destination: {}, Weight: {}", key, edge.destination, edge.weight)
        }
    }
    eulerize(&g);

    Ok(())
}

fn eulerize(graph: &HashMap<String, Node>) -> () {
    for (key, node) in graph.iter()
    {
        if (node.edges.len() % 2 == 1 )
        {
            let mut edge_to_connect : String;
            for edge in node.edges.iter() {
                // take the first node that has an odd degree and connect it, this almost certainly be optimized for weight
                if(graph[edge.destination].edges.len() % 2 == 1) 
                {
                    edge_to_connect = edge.destination.clone();
                }

                println!("Key: {}, Destination: {}, Weight: {}", key, edge.destination, edge.weight)
            }
        }

    }
}

fn find_nearest_node_with_odd_degree(node: Node, graph: &HashMap<String, Node>) -> Vec<String> {
    let mut edge_to_connect : String;
    let mut node_map: Vec<String> = Vec::new();
    for edge in node.edges.iter() {
        // take the first node that has an odd degree and connect it, this almost certainly be optimized for weight
        if(graph[edge.destination].edges.len() % 2 == 1) 
        {
            edge_to_connect = edge.destination.clone();
        }

        println!("Key: {}, Destination: {}, Weight: {}", key, edge.destination, edge.weight)
    }
    if(edge_to_connect.is_empty())
    {
        // breadth or depth first search can be used here, maybe both for most accurate results?
        // Gonna try breadth first search first

        /* get all values we have to check
        let mut frontier = graph.values().cloned().collect();

        // remove the one we started from 
        frontier.retain(|&x| x != node.name);
        */

        // get all the edges we need to traverse to

        // check to see if those edges are degree even

        // if they are, call this again, but with new nodes every time

        // if they are not, return 

    }

}

fn map_data(data: &Vec<Record>) -> HashMap<String, Node> {
    let mut g: HashMap<String, Node> = HashMap::new();
    for record in data {
        if g.contains_key(&record.edge1) {
            println!("Found Key: {}", &record.edge1);
            let e = Edge {
                source: record.edge1.clone(),
                destination: record.edge2.clone(),
                weight: record.weight.clone(),
            };
            g.get_mut(&record.edge1).unwrap().edges.push(e);
        } 
        else 
        {
            println!("No Key: {}", &record.edge1);
            let e = Edge {
                source: record.edge1.clone(),
                destination: record.edge2.clone(),
                weight: record.weight.clone(),
            };
            let mut n = Node {
                name: record.edge1.clone(),
                edges: Vec::new(),
            };
            n.edges.push(e);
            g.insert(record.edge1.clone(), n);
        }
    }
    g
}
