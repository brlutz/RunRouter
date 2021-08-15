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

    /* let csv = "node1,node2,weight
    1,2,1
    5,3,1
    1,4,1
    2,3,1
    2,5,1
    3,4,1
    4,5,1"; */

    /*let csv = "node1,node2,weight
    1,2,4
    5,3,7
    1,4,1
    2,3,2
    2,5,1
    3,4,6
    4,5,1";*/

    let csv = "node1,node2,weight
    0,1,4
    0,7,8
    1,2,8
    1,7,11
    2,3,7
    2,8,2
    2,5,4
    3,4,9
    3,5,14
    4,5,10
    5,6,2
    6,8,6
    6,7,1
    7,8,7";

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

    let d = map_to_djikstra_nodes(&vec);

    for (key, node) in d.nodes.iter()
    {
        for n in node.adj_nodes.iter() {
            println!("Key: {}, Destination: {}, Weight: {}", key, n.name, n.weight);
        }
    }

    // get odd nodes
    let mut odd_nodes: Vec<String> = Vec::new();
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

    // find maps for all the odd nodse
    let mut maps_for_odd_nodes: HashMap<String, DjikstraNodes> = HashMap::new();
    for odd_node in odd_nodes.iter() {
        let mut graph: DjikstraNodes = DjikstraNodes {
            start_node: odd_node.clone(),
            nodes: d.nodes.clone(),
        };
        find_shortest_path(&graph.start_node.clone(), &mut graph);
        maps_for_odd_nodes.insert(graph.start_node.clone(), graph);
    }

    for (key, dn) in maps_for_odd_nodes.iter()
    {
        println!("Map for odd node: {}", key);
        for (k, dnode ) in dn.nodes.iter()
        {
            println!("Traveling to node {} (out of {} nodes)",k, dn.nodes.keys().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
            let path: String = dnode.path.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
            println!("{} -> {}, Path: {}, Total Distance: {}", key, dnode.name, path, dn.nodes.get(&dnode.name).unwrap().total_distance);
        
        }
    }

    let pairs: Vec<Pair> = get_pairs(&odd_nodes);
    println!("These are the pairs");
    for pair in pairs.iter() {
        println!("{},{}", pair.node1, pair.node2);
    }


    let pair_combinations: Vec<Vec<Pair>> = get_all_pair_combinations(&pairs);
    for pair_combination in pair_combinations.iter() {
        println!("Combination");
        for combination in pair_combination
        {
            print!("{} {}, ", combination.node1, combination.node2);
        }
    }


    // get combinations of all the odd nodes 
    

    // remove duplicate combinations

    // find edges with smallest total distance

    // connect edges

    // find eulerian path. 

    Ok(())
}

fn get_all_pair_combinations(p: &Vec<Pair>) -> Vec<Vec<Pair>> {
    let mut pairs = p.clone();
    let mut results: Vec<Vec<Pair>> = Vec::new();
    let mut nodes: Vec<String> = Vec::new();

    for pair in pairs.iter() {
        let mut insert_node1: bool = true;
        let mut insert_node2: bool = true;
        if nodes.iter().any(|i| i.eq(&pair.node1)) {
            insert_node1 = false;
        }

        if nodes.iter().any(|i| i.eq(&pair.node2)) {
            insert_node2 = false;
        }
        
        if insert_node1 {nodes.push(pair.node1.clone());}
        if insert_node2 {nodes.push(pair.node2.clone());}
    }

    for pair in pairs.iter() {
        println!("#### getting pair combinations starting with {} {}", pair.node1, pair.node2);
        results.push(get_pair_combinations(p, nodes.clone()))
    }

    return results;
}

fn get_pair_combinations(p: &Vec<Pair>, mut n: Vec<String>) -> Vec<Pair> {
    let mut result: Vec<Pair> = Vec::new();
    let mut pairs = p.clone();
    let mut nodes = n.clone();
    for pair in pairs.iter() {
        println!("Checking pair {} {}, with nodes left {}", pair.node1, pair.node2, nodes.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
        // if the node contains both "unused" values, add it to a list
        if nodes.iter().any(|i| i.eq(&pair.node1)) && nodes.iter().any(|i| i.eq(&pair.node2)) {
            // add the pair
            result.push(pair.clone());
            // remove the nodes from the acceptable list
            nodes.retain(|x| !x.eq(&pair.node1) );
            nodes.retain(|x| !x.eq(&pair.node2));
            println!("Pushing pair {} {}, now nodes left are {}", pair.node1, pair.node2, nodes.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
            if nodes.len() > 0 {
                println!("####recursing");
                result.append(&mut get_pair_combinations(p, nodes.clone()))
            }
        } else {
            println!("Pair {} {} has used values, skipping", pair.node1, pair.node2);
        }
    }

    return result

}

fn get_pairs(n: &Vec<String>) -> Vec<Pair> {
    
    if n.len() % 2 != 0 { panic!("There should never be an odd number of pair options");}

    let mut names = n.clone();
    let mut results: Vec<Pair> = Vec::new();

    for node1 in names.iter(){
        println!("looking for pairs for {}", node1);
        for node2 in names.iter() {
            println!("## does that pair with {}", node2);
            if node1.eq(node2) {continue;}
            let mut should_insert: bool = true;
            let mut p = Pair {node1:node1.clone(), node2:node2.clone()};

            
            for result in &results {
                // filter out equivilent dupes
                println!("### Compairing result {}, {}, and pair {}, {} ", result.node1, result.node2, p.node1, p.node2);
                if (result.node1.eq(node1) && result.node2.eq(node2)) || (result.node1.eq(node2) && result.node2.eq(node1)) {
                    println!("### Found similar node, not going to insert"); 
                    should_insert = false;
                    break;
                }
            }

            if should_insert {
                println!("# pushing pair {},{}", p.node1, p.node2);
                results.push(p);
            }
        }
    }


   return results
}

fn are_combinations_equal(set1: Vec<Pair>, set2: Vec<Pair>) -> bool {
    false
}

#[derive(Clone)]
struct Pair {
    node1: String,
    node2: String,
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


fn find_shortest_path(start: &String, graph: &mut DjikstraNodes) -> () {
    // Done in earlier function: for nodes, mark node dist(0), rest dist(infinity)

    //println!("There are {} DjikstraNodes", graph.nodes.keys().len());

    if start.eq(&graph.start_node) {
        let mut node_being_looked_at: DjikstraNode = graph.nodes.remove(start).unwrap();
        node_being_looked_at.total_distance = 0;
        node_being_looked_at.path.push(start.clone());
        //println!("Start {} and start node {} are equal. Path: {}", start, &graph.start_node, node_being_looked_at.path.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
        graph.nodes.insert(start.clone(), node_being_looked_at);
    }

    let current_node_name = start;
    //println!("Starting search on node {}", current_node_name);

    let mut current_node = graph.nodes.remove(current_node_name).unwrap();

    let mut name_of_closest_node:String = "".to_string();
    let mut distance_of_closest_node: u16 = u16::MAX;
    for node in current_node.adj_nodes.iter() {
        if graph.nodes.get(&node.name).unwrap().traversed { 
            //println!("Node {} has already been traveled", &node.name);
            continue;
         }
        let mut node_being_looked_at: DjikstraNode = graph.nodes.remove(&node.name).unwrap();
        println!("Looking at node {}", node_being_looked_at.name);
        // for node update edges with total distance from node if distance less than existing distance
            // update nodes edges with map from node -> neighbors
        if node.weight+current_node.total_distance < node_being_looked_at.total_distance {
            node_being_looked_at.total_distance = node.weight+current_node.total_distance;
            if node_being_looked_at.path.len() == 0 {
                
                for p in current_node.path.iter() {
                    node_being_looked_at.path.push(p.clone());
                }
                node_being_looked_at.path.push(node.name.clone());
                //println!("Node being looked at path was empty, is now {}", node_being_looked_at.path.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
            }
            else
            {
                node_being_looked_at.path = current_node.path.clone();
                node_being_looked_at.path.push(node.name.clone());
                //println!("Node being looked at path was not empty, is now {}", node_being_looked_at.path.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));

            }
        } else {
            //println!("node weight {} was not less than node being looked at total distance {}", node.weight, node_being_looked_at.total_distance);
        }


        graph.nodes.insert(node.name.to_string(), node_being_looked_at);

        if !graph.nodes.get(&node.name).unwrap().traversed && graph.nodes.get(&node.name).unwrap().total_distance < distance_of_closest_node {
            distance_of_closest_node = graph.nodes.get(&node.name).unwrap().total_distance;
            name_of_closest_node = node.name.clone();
        }
    }

    current_node.traversed = true;
    graph.nodes.insert(current_node_name.to_string(), current_node);

    //println!("Found closest node {} with distance of {}", name_of_closest_node, distance_of_closest_node);

    // recurse
    if !name_of_closest_node.is_empty() {
        // println!("Found a closest node {}, recursing", name_of_closest_node);
        find_shortest_path(&name_of_closest_node.clone(), graph);
    }
}

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