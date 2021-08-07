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
            record.edge1.trim().to_string(),
            record.edge2.trim().to_string(),
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



fn connect_nodes_via_map(node_map: Vec<String>, graph: &HashMap<String, Node>) -> Option<&HashMap<String, Node>> {

    return None

}

fn connect_nodes(start: String, end: String, weight: u16, g: HashMap<String, Node>) -> HashMap<String, Node> {
    let mut graph = g;
    if graph.contains_key(&start) {
        // println!("Found Key: {}", &start);
        let e = Edge {
            source: start.clone(),
            destination: end.clone(),
            weight: weight,
        };
        graph.get_mut(&start).unwrap().edges.push(e);
    } 
    else 
    {
        // println!("No Key: {}", &start);
        let e = Edge {
            source: start.clone(),
            destination: end.clone(),
            weight: weight.clone(),
        };
        let mut n = Node {
            name: start.clone(),
            edges: Vec::new(),
        };
        n.edges.push(e);
        graph.insert(start.clone(), n);
    }

    return graph
}

fn eulerize(graph: &HashMap<String, Node>) -> Option<&HashMap<String, Node>> {
    if !is_eulerized(&graph) {
        println!("Graph is not eulerized");
        for (key, node) in graph.iter()
        {                
            // find a node that's odd
            if node.edges.len() % 2 == 1 
            {
                let mut success: bool = false;
                let mut node_map: Vec<String> = Vec::new();
                let mut traversed: Vec<String> = Vec::new();

                // find the nearest odd node
                match find_path_to_node_with_odd_degree(node.name.clone(), graph, &mut traversed ) {
                    Some(mut x) => 
                    {
                        success = true;
                        node_map.append(&mut x);
                        break;
                    }
                    None => { println!("Hit a dead end")}
                }

                // connect the two nodes


                /*
                let mut edge_to_connect : String;
                for edge in node.edges.iter() {
                    // take the first node that has an odd degree and connect it, this almost certainly be optimized for weight
                    if graph[&edge.destination].edges.len() % 2 == 1
                    {
                        edge_to_connect = edge.destination.clone();
                    }
    
                    println!("Key: {}, Destination: {}, Weight: {}", key, edge.destination, edge.weight)
                }
                */
            }
    
        }
    }

    println!("Graph is eulerized");


    return None
    //return graph
}

fn is_eulerized(graph: &HashMap<String, Node>) -> bool {
   let mut count = 0;
    for (key, node) in graph.iter()
    {
        if node.edges.len() % 2 == 1 
        {
            count = count+1;
        }
    }

    // traversable only if 2 or 0 because math
    if count == 2 || count == 0  {return true}
    return false
}

fn find_path_to_node_with_odd_degree(node_name: String, graph: &HashMap<String, Node>, traversed: &mut Vec<String>) -> Option<Vec<String>> {
    let mut edge_to_connect : String = "".to_string();
    let mut node_map: Vec<String> = Vec::new();
    let mut success: bool = false;

    // We've already been here, abort
    if traversed.contains(&node_name) {
       return None
    }

    // breadth first search of the current node
    for edge in graph[&node_name].edges.iter() {
        // take the first node that has an odd degree and connect it, this almost certainly be optimized for weight
        if graph[&edge.source].edges.len() % 2 == 1
        {
            edge_to_connect = edge.destination.clone();
            break;
        }

        // println!("Key: {}, Destination: {}, Weight: {}", key, edge.destination, edge.weight)
    }

    // start breadth first searches of adjacent nodes
    if edge_to_connect.is_empty()
    {

        // breadth or depth first search can be used here, maybe both for most accurate results?
        // Gonna try breadth first search first

        /* get all values we have to check
        let mut frontier = graph.values().cloned().collect();

        // remove the one we started from 
        frontier.retain(|&x| x != node.name);
        */

        traversed.push(node_name.clone());

        // get all the edges we need to traverse to
        for edge in graph[&node_name].edges.iter() {

            match find_path_to_node_with_odd_degree(edge.source.clone(), graph, traversed) {
                Some(mut x) => 
                {
                    success = true;
                    node_map.append(&mut x);
                    break;
                }
                None => { println!("Hit a dead end")}
            }
        }

        if success {
            return Some(node_map)
        }

        return None

    }

   return Some(node_map)

}

fn map_data(data: &Vec<Record>) -> HashMap<String, Node> {
    
    let mut g: HashMap<String, Node> = HashMap::new();
    for record in data {

        // undirectional, so you have to do both.
        g = connect_nodes(record.edge1.trim().to_string(), record.edge2.trim().to_string(), record.weight, g);
        g = connect_nodes(record.edge2.trim().to_string(), record.edge1.trim().to_string(), record.weight, g);
    }
   return g
}
