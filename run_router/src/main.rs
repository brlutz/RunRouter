use serde::Deserialize;
use std::collections::HashMap;
// use std::error::Error;

#[derive(Deserialize)]
struct Record {
    node1: String,
    node2: String,
    weight: u16,
}
#[derive(Clone)]
struct Node {
    name: String,
    edges: Vec<Edge>,
}

#[derive(Clone)]
struct Edge {
    node1: String,
    node2: String,
    weight: u16,
    traversed: bool
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

    let csv = "node1,node2,weight
    1,2,4
    5,3,7
    1,4,1
    2,3,2
    2,5,1
    3,4,6
    4,5,1";

    /*let csv = "node1,node2,weight
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
    7,8,7";*/

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
    let mut nodes: HashMap<String, Node> = map_nodes(&vec);
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

    // find maps for all the odd nodes
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

    // find all the possible pairs
    let pairs: Vec<Pair> = get_pairs(&odd_nodes);
    println!("These are the pairs");
    for pair in pairs.iter() {
        println!("{},{}", pair.node1, pair.node2);
    }

    // find all possible pair combinations
    let pair_combinations: Vec<Vec<Pair>> = get_all_pair_combinations(&pairs);

    // get combinations costs
    let mut costs: Vec<u16> = Vec::new();
    for pair_combination in pair_combinations.iter() {
        println!("\n Combination");
        let mut cost = 0;
        for pair in pair_combination.iter() {
            print!("{} {}, ", pair.node1, pair.node2);
            cost += maps_for_odd_nodes.get(&pair.node1).unwrap().nodes.get(&pair.node2).unwrap().total_distance 
        }   
        costs.push(cost);
        print!(" Cost: {} ", cost);
    }

    // find location of edges with smallest total distance
    let mut cheapest_value = u16::MAX;
    let mut cheapest_index = 0;
    for i in 0..(costs.len()-1) {
        if costs[i] < cheapest_value {
            cheapest_index = i;
            cheapest_value = costs[i];
        }
    }

    println!("Cheapest is at {} with cost {}", cheapest_index, cheapest_value);

    // get cheapest edges
    let cheapest_pairs = pair_combinations[cheapest_index].to_owned();
    println!("Cheapest combination is:");
    for pair in cheapest_pairs.iter() {
        print!("{} {}, ", pair.node1, pair.node2);
    }   
    println!("With cost: {}", cheapest_value);
    
    
    // connect edges
    for pair in cheapest_pairs.iter() {
        println!("Going to connect {}->{} ", pair.node1, pair.node2);
        let path: Vec<String> = maps_for_odd_nodes.get(&pair.node1).unwrap().nodes.get(&pair.node2).unwrap().path.clone();
        println!("Path is {}", path.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
        for i in 0..path.len()-1 {
            println!("i is {}", i);
            let weight = edges.iter().find(|&x| (x.node1 == path[i] && x.node2 == path[i+1]) || (x.node2 == path[i] && x.node1 == path[i+1])).unwrap().weight;
            // let weight = maps_for_odd_nodes.get(&path[i].clone()).unwrap().nodes.get(&path[i+1].clone()).unwrap().adj_nodes.iter().find(|&x| x.name == path[i+1]).unwrap().weight;
            // let weight = nodes.get(&path[i].clone()).unwrap().edges.iter().find(|&x| x.node2 == path[i+1]).unwrap().weight.clone();
            nodes = connect_nodes(path[i].clone(),path[i+1].clone(), weight, nodes );
            nodes = connect_nodes(path[i+1].clone(),path[i].clone(), weight, nodes );
        }
    }  


    for (key, node) in nodes.iter()
    {
        for edge in node.edges.iter() {
            println!("Key: {}, Destination: {}, Weight: {}", key, edge.node2, edge.weight)
        }
    }

    // find eulerian path. 
    let map_result = find_eulerian_circuit(&mut nodes, "1".to_string());

    println!("Distance {}, map: {}", map_result.weight, map_result.map.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));


    Ok(())
}


fn find_eulerian_circuit(nodes: &mut HashMap<String, Node>, start_node_name: String) -> MapResult {
    

    let mut result_map:MapResult = MapResult {
        map: Vec::new(),
        weight: 0
    };

    // let mut nodes = n.clone();
    let mut current_node_name = start_node_name.clone();
    loop {
        let mut next_map = MapResult {
            map: Vec::new(),
            weight: 0,
        };
        // hierholtzer algo
        // random walk among non traversed edges to find path
        
        loop {

            let mut current_node = nodes.remove(&current_node_name).unwrap();
            let current_node_name = &current_node.name.clone();
            let mut next_node_name = "BAD NODE".to_string();
            let edges_len = current_node.edges.len();

            for i in 0..edges_len-1 {
                // for each vert in path, check to see if edges are untravelled
                if current_node.edges[i].clone().traversed {continue;}

                // if they're untraveled, take them and travel until back at starting

                // mark original edge as traversed
                current_node.edges[i].traversed = true;
                next_node_name = current_node.edges[i].node2.clone();
                nodes.insert(current_node_name.clone(), current_node.to_owned());

                // update the map the first time
                if next_map.map.len() == 0
                {
                    next_map.map.push(current_node_name.clone());
                }

                // update corresponding edge in the destination node
                
                let mut node_to_update_edge = nodes.remove(&next_node_name).unwrap();
                let position_of_edge_to_update = node_to_update_edge.edges.iter().position(|x| x.node1 == current_node.edges[i].node2 && x.node2 == current_node.edges[i].node1).unwrap();
                let weight_of_edge = node_to_update_edge.edges[position_of_edge_to_update].weight;
                node_to_update_edge.edges[position_of_edge_to_update].traversed = true;
                nodes.insert(node_to_update_edge.name.clone(), node_to_update_edge);

                // update the map the first time
                if next_map.map.len() == 0
                {
                    next_map.map.push(current_node_name.clone());
                    
                } else 
                {
                    next_map.map.push(next_node_name.clone());
                }
                next_map.weight = next_map.weight + weight_of_edge;
                break;
            
            }

            // # if they're untraveled, take them and travel until back at starting
            if next_node_name.eq(&start_node_name) { break; }

        }

        // merge maps
        let result_map_size = result_map.map.len();
        if result_map_size> 0 {
            for i in 0..result_map.map.len() - 1 {
                match next_map.map.iter().find(|&x| x.eq(&result_map.map[i])) {
                Some (x) => {
                    result_map.map.splice(i..i, next_map.map.clone());
                }
                None => {
                    panic!("Should never not find something in the results map we have in the next map, this means disconnected graphs!");
                }
                }
            }
         }
         else {
             result_map = next_map;
         }
        
        // find a new start node
        for (key, node) in nodes.iter_mut()
        {
            let mut is_valid_node = false;
            let mut untraversed_node: Option<String> = None;
            for e in node.edges.iter() {
                if e.traversed {
                    is_valid_node = true;
                    continue ;
                }
                println!("Traveling to the {} node", e.node1);
                

                untraversed_node = Some(e.node1.clone());
            }
            if is_valid_node {
                match untraversed_node {
                    Some (x) => {
                        current_node_name = x;
                    }
                    None => {}
                }
            }
        }

        break;
    }
    // repeat
    // splice the map in 

    return result_map
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
       // println!("#### getting pair combinations starting with {} {}", pair.node1, pair.node2);
        let combinations: Vec<Pair> = get_pair_combinations(p, nodes.clone(), Some(pair.clone()));
        
        let mut should_insert_combination: bool = true;
        for result in results.iter() {
            
            if are_sets_of_pairs_eqivilent(result, &combinations) {
                should_insert_combination = false;
                break;
            }
        }
        if should_insert_combination {
            results.push(combinations);
        }
    }

    return results;
}

fn are_sets_of_pairs_eqivilent(one_pairs: &Vec<Pair>, two_pairs: &Vec<Pair>) -> bool {
    
    let result: bool = false;
    if one_pairs.len() != two_pairs.len() { return false;}

    for one in one_pairs {
        let mut found: bool = false;
        for two in two_pairs {
            if (one.node1 == two.node1 && one.node2 == two.node2) || (one.node1 == two.node2 && one.node2 == two.node1){
                found = true;
            }
        }
        if !found { return false; }
    }

    return true;
}

fn get_pair_combinations(p: &Vec<Pair>, mut n: Vec<String>, start_pair: Option<Pair>) -> Vec<Pair> {
    let mut result: Vec<Pair> = Vec::new();
    let mut pairs = p.clone();
    let mut nodes = n.clone();

    match start_pair {
        Some(selected_start_pair) => 
        {
            // println!("Found a start pair");

            // println!("Checking pair {} {}, with nodes left {}",selected_start_pair.node1, selected_start_pair.node2, nodes.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));

            // if the node contains both "unused" values, add it to a list
            if nodes.iter().any(|i| i.eq(&selected_start_pair.node1)) && nodes.iter().any(|i| i.eq(&selected_start_pair.node2)) {
                // add the pair
                result.push(selected_start_pair.clone());
                // remove the nodes from the acceptable list
                nodes.retain(|x| !x.eq(&selected_start_pair.node1) );
                nodes.retain(|x| !x.eq(&selected_start_pair.node2));
               // println!("Pushing pair {} {}, now nodes left are {}", selected_start_pair.node1, selected_start_pair.node2, nodes.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
                if nodes.len() > 0 {
                    // println!("####recursing");
                    for pair in pairs.iter() {
                        result.append(&mut get_pair_combinations(p, nodes.clone(), Some(pair.clone())))
                    }
                    
                }
            } else {
               //  println!("Pair {} {} has used values, skipping", selected_start_pair.node1, selected_start_pair.node2);
            }
        
        } 
        None => {
            panic!("We always should have a start pair");
        }
    };

    return result

}

fn get_pairs(n: &Vec<String>) -> Vec<Pair> {
    
    if n.len() % 2 != 0 { panic!("There should never be an odd number of pair options");}

    let mut names = n.clone();
    let mut results: Vec<Pair> = Vec::new();

    for node1 in names.iter(){
        // println!("looking for pairs for {}", node1);
        for node2 in names.iter() {
            // println!("## does that pair with {}", node2);
            if node1.eq(node2) {continue;}
            let mut should_insert: bool = true;
            let mut p = Pair {node1:node1.clone(), node2:node2.clone()};

            
            for result in &results {
                // filter out equivilent dupes
                 // println!("### Compairing result {}, {}, and pair {}, {} ", result.node1, result.node2, p.node1, p.node2);
                if (result.node1.eq(node1) && result.node2.eq(node2)) || (result.node1.eq(node2) && result.node2.eq(node1)) {
                    // println!("### Found similar node, not going to insert"); 
                    should_insert = false;
                    break;
                }
            }

            if should_insert {
                // println!("# pushing pair {},{}", p.node1, p.node2);
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
        // println!("Looking at node {}", node_being_looked_at.name);
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
            traversed: true,
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
            traversed: false
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
            traversed: false,
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