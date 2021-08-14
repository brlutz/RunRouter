use serde::Deserialize;
use std::collections::HashMap;
// use std::error::Error;

#[derive(Deserialize)]
struct Record {
    node1: String,
    node2: String,
    weight: u16,
}

struct Node {
    name: String,
    edges: Vec<Edge>,
}

struct Edge {
    node1: String,
    node2: String,
    weight: u16,
}

struct MapResult {
    map: Vec<String>,
    weight: u16
}

fn get_data() -> Result<Vec<Record>, csv::Error> {

    let csv = "node1,node2,weight
    1,2,1
    5,3,1
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
            "Nodes: {},{}: Weight: {}.",
            record.node1,
            record.node2,
            record.weight,
        );
    }
    
    let edges = map_data(&vec);
    map_nodes(&vec);
    let d = map_to_djikstra_nodes(&vec);
    
    /*
    for (key, node) in nodes.iter()
    {
        for edge in node.edges.iter() {
            println!("Key: {}, Destination: {}, Weight: {}", key, edge.node2, edge.weight)
        }
    }
    */

    for (key, node) in d.nodes.iter()
    {
        for n in node.adj_nodes.iter() {
            println!("Key: {}, Destination: {}, Weight: {}", key, n.name, n.weight)
        }
    }

    let mut odd_nodes: Vec<String> = Vec::new();
    //find_route(&g);
    match is_eulerized(&edges) {
        Some(x) => 
        {
            println!("Is Not eulerized");
            for node in x.iter()
            {
              println!("Odd degree nodes: {}", node);
            }
            odd_nodes = x;
         
             // println!("Graph has {} odd degrees", x.len());
        } 
        None => { println!("Is eulerized");}
    };

    let mut maps_for_odd_nodes: HashMap<String, DjikstraNodes> = HashMap::new();

    for odd_node in odd_nodes {
        maps_for_odd_nodes.insert(odd_node.clone(), find_shortest_path(&odd_node, &d));
        break;
    }

    for (key, dn) in maps_for_odd_nodes.iter()
    {
        for (k, dnode ) in dn.nodes.iter()
        {
            for n in dnode.adj_nodes.iter() {
                let path: String = dnode.path.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
                println!("Start Node: {}, Destination: {}, Path: {}, Path Weight: {}", key, n.name, path, n.weight)
            }

        }
    }

    Ok(())
}

#[derive(Clone)]
struct DjikstraNode {
    name: String,
    total_distance: u16,
    path: Vec<String>,
    traversed: bool,
    adj_nodes: Vec<NodeWeightMap>,
}

struct DjikstraNodes {
    start_node: String,
    nodes: HashMap<String, DjikstraNode>,
}

#[derive(Clone,)]
struct NodeWeightMap {
    name: String,
    weight: u16
}


fn find_shortest_path(start: &String, g: &DjikstraNodes) -> DjikstraNodes {
    // Done in earlier function: for nodes, mark node dist(0), rest dist(infinity)

    let mut graph: DjikstraNodes = DjikstraNodes {
        start_node: g.start_node.clone(),
        nodes: g.nodes.clone(),
    };

    if start.eq(&graph.start_node) {
        let mut node_being_looked_at: DjikstraNode = graph.nodes.remove(start).unwrap();
        node_being_looked_at.total_distance = 0;
        node_being_looked_at.path.push(start.clone());
    }

    let current_node_name = start;

    let mut current_node = graph.nodes.remove(current_node_name).unwrap();


    for node in current_node.adj_nodes.iter() {
        let mut node_being_looked_at: DjikstraNode = graph.nodes.remove(&node.name).unwrap();
        
        // for node update edges with total_distance from node if distance less than existing distance
            // update nodes edges with map from node -> neighbors
        if node.weight < node_being_looked_at.total_distance {
            node_being_looked_at.total_distance = node.weight;
            if node_being_looked_at.path.len() == 0 {
                node_being_looked_at.path.push(current_node_name.clone());
                node_being_looked_at.path.push(node.name.clone());
            }
            else
            {
                node_being_looked_at.path = graph.nodes.get_mut(current_node_name).unwrap().path.clone();
                node_being_looked_at.path.push(node.name.clone());

            }
        }
        graph.nodes.insert(node.name.to_string(), node_being_looked_at);
    }

    current_node.traversed = true;
    graph.nodes.insert(current_node_name.to_string(), current_node);

    let mut name_of_closest_node:String = "".to_string();
    let mut distance_of_closest_node: u16 = u16::MAX;
    for (_key, node) in graph.nodes.iter()
    {
        if !node.traversed && node.total_distance < distance_of_closest_node {
            distance_of_closest_node = node.total_distance;
            name_of_closest_node = node.name.clone();
        }

    }

    // recurse
    if !name_of_closest_node.is_empty() {
        graph = find_shortest_path(&name_of_closest_node.clone(), &graph);
    }

    return graph
    

    // mark node as traversed
    // find non traversed node, recurse

    // when all nodes are complete, map total distance and path to djikstra node
    // return string, distance, map
}

/*
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

fn eulerize(graph: &Vec<Record>) -> Option<&HashMap<String, Node>> {
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
*/
fn is_eulerized(graph: &Vec<Edge>) -> Option<Vec<String>> {

   let mut nodes: Vec<String> = Vec::new();
   let mut odd_nodes: Vec<String> = Vec::new();
   let mut odd_count: u16 = 0;
   for record in graph.iter() 
   {
     if !nodes.contains(&record.node1)
     {
         nodes.push(record.node1.clone())
     }

     if !nodes.contains(&record.node2)
     {
         nodes.push(record.node2.clone())
     }
   }

   for node in nodes.iter()
   {
        let mut count = 0;

        for record in graph.iter() {
            if &record.node1 == node || &record.node2 == node {
                count = count+1;
            }
        }

        if count % 2 == 1 {
            odd_count = odd_count + 1;
            odd_nodes.push(node.clone());
        }
   } 

    // traversable only if 2 or 0 because math
    if odd_count == 0  {    return None}

    return Some(odd_nodes)
}

/*
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
*/


fn map_data(data: &Vec<Record>) -> Vec<Edge> {
    
    let mut g: Vec<Edge> = Vec::new();
    for record in data {
        let e: Edge = Edge {
            node1: record.node1.trim().to_string(),
            node2: record.node2.trim().to_string(),
            weight: record.weight,
        };

        println!(
            "Nodes: {},{}, Weight: {} ",
            e.node1,
            e.node2,
            e.weight,
        );
        g.push(e);

    }
   return g
}

fn map_nodes(data: &Vec<Record>) -> HashMap<String, Node> {
    
    let mut g: HashMap<String, Node> = HashMap::new();
    for record in data {

        // undirectional, so you have to do both.
        g = connect_nodes(record.node1.trim().to_string(), record.node2.trim().to_string(), record.weight, g);
        g = connect_nodes(record.node2.trim().to_string(), record.node1.trim().to_string(), record.weight, g);
    }
   return g
}

fn map_to_djikstra_nodes(data: &Vec<Record>) -> DjikstraNodes {
    
    let mut g : DjikstraNodes = DjikstraNodes {
        start_node: "".to_string(),
        nodes: HashMap::new(),
    };

    for record in data {
        // undirectional, so you have to do both.
        g = connect_djikstra_nodes(record.node1.trim().to_string(), record.node2.trim().to_string(), record.weight, g);
        g = connect_djikstra_nodes(record.node2.trim().to_string(), record.node1.trim().to_string(), record.weight, g);
    }
   return g
}

fn connect_nodes(start: String, end: String, weight: u16, g: HashMap<String, Node>) -> HashMap<String, Node> {
    let mut graph = g;
    if graph.contains_key(&start) {
        // println!("Found Key: {}", &start);
        let e = Edge {
            node1: start.clone(),
            node2: end.clone(),
            weight: weight,
        };
        graph.get_mut(&start).unwrap().edges.push(e);
    } 
    else 
    {
        // println!("No Key: {}", &start);
        let e = Edge {
            node1: start.clone(),
            node2: end.clone(),
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

fn connect_djikstra_nodes(start: String, end: String, weight: u16, g: DjikstraNodes, ) -> DjikstraNodes {
    let mut graph = g;
    if graph.nodes.contains_key(&start) {
        // println!("Found Key: {}", &start);
        let e = NodeWeightMap {
            name: end.clone(),
            weight: weight,
        };
        graph.nodes.get_mut(&start).unwrap().adj_nodes.push(e);
    } 
    else 
    {
        // println!("No Key: {}", &start);
        let e = NodeWeightMap {
            name: end.clone(),
            weight: weight.clone(),
        };
        let mut n = DjikstraNode {
            total_distance: u16::MAX,
            path: Vec::new(),
            adj_nodes: Vec::new(),
            traversed: false,
            name: start.clone(),
        };
        n.adj_nodes.push(e);
        graph.nodes.insert(start.clone(), n);
    }

    return graph
}