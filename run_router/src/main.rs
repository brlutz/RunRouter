use rand::Rng;
use serde::Deserialize;
use std::collections::HashMap;
use rand::SeedableRng;
//use std::time::{SystemTime, UNIX_EPOCH};
// use std::error::Error;

#[derive(Deserialize)]
struct Record {
    node1: String,
    node2: String,
    weight: f32,
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
    weight: f32,
    traversed: bool,
}

struct MapResult {
    map: Vec<String>,
    weight: f32,
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

    /*let csv = "node1,node2,weight
    A,B,1
    A,C,1
    B,C,1
    B,D,1
    B,E,1
    C,D,1
    C,E,1
    D,E,1
    D,F,1
    E,F,1";*/

    let csv = "node1,node2,weight
WillowWay/Creek, Timberline/Willow,0.064
Timberline/Willow, Timberline/Poplar,0.262
Timberline/Poplar, Timberline/Maple,0.06
Timberline/Maple, Timberline/HickoryS,0.128
Timberline/HickoryS, Timberline/HickoryN,0.22
Timberline/HickoryS, Timberline/HickoryN,0.223
Timberline/HickoryN, Timberline/Hemlock,0.093
Timberline/Hemlock, Timberline/Magnolia,0.076
Timberline/Magnolia, Timberline/Rancocas,0.11
Timberline/Rancocas, Rancocas/Cedar,0.068
Rancocas/Cedar, Rancocas/Ash,0.132
Cedar/Maple, Timberline/Rancocas,0.150
Rancocas/Ash, Rancocas/Evergreen,0.116
Rancocas/Evergreen, Rancocas/Overhill,0.067
Rancocas/Overhill, Rancocas/WoodLane,0.059
Rancocas/WoodLane, Beach/Pine,0.14
Beach/Pine, Rancocas/Pine,0.097
Rancocas/Pine, Rancocas/WoodLane,0.094
Rancocas/Pine, Rancocas/Oak,0.059
Beach/Oak, Rancocas/Oak,0.113
Rancocas/Oak, Rancocas/Lake,0.06
Rancocas/Lake, Rancocas/Creek,0.083
Rancocas/Creek, Creek/Woolman,0.2
Creek/Woolman, Creek/Conestoga,0.173
Creek/Woolman, Holly/Conestoga,0.086
WillowWay/Creek, Creek/Conestoga,0.18
Beach/Pine, Beach/Oak,0.054
Beach/Oak, Rancocas/Lake,0.182
Timberline/Willow, Larch/Holly,0.062
Larch/Holly, Evergreen/Larch,0.063
Evergreen/Larch, Larch/Linden,0.061
Larch/Linden, Poplar/Larch,0.132
Poplar/Larch, Timberline/Poplar,0.069
Timberline/Maple, Hemlock/Maple,0.081
Hemlock/Maple, Timberline/Hemlock,0.27
Hemlock/Maple, Magnolia/Maple,0.056
Magnolia/Maple, Timberline/Magnolia,0.235
Cedar/Maple, Maple/Walnut,0.0522
Maple/Walnut, Maple/Linden,0.038
Maple/Linden, Larch/Maple,0.047
Larch/Maple, Magnolia/Maple,0.027
Creek/Conestoga, Holly/Conestoga,0.088
Holly/Conestoga, Larch/Holly,0.105
Conestoga/Woolman, Evergreen/Woolman,0.144
Holly/Conestoga, Conestoga/Woolman,0.012
Evergreen/Larch, Evergreen/Woolman,0.069
Cedar/Maple, Ash/Cedar,0.045
Overhill/Evergreen/Walnut, Overhill/UpperPark,0.084
Overhill/UpperPark, WestWood/UpperPark,0.062
Rancocas/Oak, LowerParkRoad,0.09
Rancocas/Lake, LowerParkRoad,0.09
Rancocas/WoodLane, WestWood/UpperPark,0.062
Rancocas/Overhill, Overhill/UpperPark,0.063
Larch/Linden, Maple/Linden,0.207
Overhill/Evergreen/Walnut, Maple/Walnut,0.189
Overhill/Evergreen/Walnut, Evergreen/Woolman,0.065
Ash/Cedar, Evergreen/Cedar,0.119
Evergreen/Cedar, Rancocas/Evergreen,0.068
Rancocas/Ash, Ash/Cedar,0.068
Poplar/Larch, Larch/Maple,0.098
Overhill/Evergreen/Walnut, Evergreen/Cedar,0.068"; 
/*
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
    7,8,7";*/

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let mut vec: Vec<Record> = Vec::new();
    for record in reader.deserialize() {
        let record: Record = record?;
        println!("{}, {}, {}", record.node1, record.node2, record.weight);
        vec.push(record);
    }

    Ok(vec)
}

fn main() -> Result<(), csv::Error> {
    let vec: Vec<Record> = get_data().unwrap();
    // print_graphml(&vec);
    for record in &vec {
        println!(
            "Nodes: {},{}: Weight: {}.",
            record.node1, record.node2, record.weight,
        );
    }
    let edges = map_data(&vec);
    let mut nodes: HashMap<String, Node> = map_nodes(&vec);
    let d = map_to_djikstra_nodes(&vec);

    for (key, node) in d.nodes.iter() {
        for n in node.adj_nodes.iter() {
            println!(
                "Key: {}, Destination: {}, Weight: {}",
                key, n.name, n.weight
            );
        }
    }

    // get odd nodes
    let mut odd_nodes: Vec<String> = Vec::new();
    let mut is_eulered = false;
    match is_eulerized(&edges) {
        Some(x) => {
            println!("Is Not eulerized, there are {} odd nodes", x.len());
            for node in x.iter() {
                println!("Odd degree nodes: {}", node);
            }
            odd_nodes = x;

            // println!("Graph has {} odd degrees", x.len());
        }
        None => {
            println!("Is eulerized");
            is_eulered = true;
        }
    };
    if !is_eulered {
        // find maps for all the odd nodes
        let mut maps_for_odd_nodes: HashMap<String, DjikstraNodes> = HashMap::new();
        let mut count = 0;
        for odd_node in odd_nodes.iter() {
            println!("Prepping node {}", odd_node);
            let mut graph: DjikstraNodes = DjikstraNodes {
                start_node: odd_node.clone(),
                nodes: d.nodes.clone(),
                median_total_distance: 0.0
            };
            find_shortest_path(&graph.start_node.clone(), &mut graph);
            find_median_distance(&graph.start_node.clone(), &mut graph);
            maps_for_odd_nodes.insert(graph.start_node.clone(), graph);
            count = count + 1;
            if count == 2 {panic!("We're taking a break here")}
        }

        for (key, dn) in maps_for_odd_nodes.iter() {
            // println!("Map for odd node: {}", key);
            for (k, dnode) in dn.nodes.iter() {
               println!(
                    "Traveling to node {} (out of {} nodes)",
                    k,
                    dn.nodes
                        .keys().len()
                        /*.map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(",")*/
                );
                let path: String = dnode
                    .path
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
              println!(
                    "{} -> {}, Path: {}, Total Distance: {}",
                    key,
                    dnode.name,
                    path,
                    dn.nodes.get(&dnode.name).unwrap().total_distance
                );
            }
        }
        // find all the possible pairs
        let mut pairs: Vec<Pair> = get_pairs(&odd_nodes);
        println!("These are the pairs. There are {}", pairs.len());
        for pair in pairs.iter() {
            println!("{},{}", pair.node1, pair.node2);
        }

        let MAX_PAIR_OPTIONS:usize = 3;
        pairs = get_pairs_distance(&pairs, &maps_for_odd_nodes);

        // pairs = get_optimized_pairs(&pairs);
        println!("These are the optimized pairs. There are {}", pairs.len());
        for pair in pairs.iter() {
            println!("{},{}", pair.node1, pair.node2);
        }
        //panic!("gonna check this out first");
        // find all possible pair combinations
        let pair_combinations: Vec<Vec<Pair>> = get_all_pair_combinations(&pairs, &maps_for_odd_nodes);
        
        
        //if optimized_pair_combinations.len() < 1 { panic!("There were no optimized combinations found");}

        // get combinations costs
        let mut costs: Vec<f32> = Vec::new();
        for pair_combination in pair_combinations.iter() {
            println!("\n Combination");
            let mut cost = 0.0;
            for pair in pair_combination.iter() {
                let single_cost = maps_for_odd_nodes
                .get(&pair.node1)
                .unwrap()
                .nodes
                .get(&pair.node2)
                .unwrap()
                .total_distance;
                cost += single_cost;
                print!("{}, {}, cost: {}. ", pair.node1, pair.node2, single_cost);
            }
            costs.push(cost);
            print!("Total Cost: {} ", cost);
        }
        // find location of edges with smallest total distance
        let mut cheapest_value = f32::MAX;
        let mut cheapest_index = 0;
        for i in 0..(costs.len() - 1) {
            if costs[i] < cheapest_value {
                cheapest_index = i;
                cheapest_value = costs[i];
            }
        }

        println!(
            "Cheapest is at {} with cost {}",
            cheapest_index, cheapest_value
        );

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
            let path: Vec<String> = maps_for_odd_nodes
                .get(&pair.node1)
                .unwrap()
                .nodes
                .get(&pair.node2)
                .unwrap()
                .path
                .clone();
            println!(
                "Path is {}",
                path.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );
            for i in 0..path.len() - 1 {
                //println!("i is {}", i);
                let weight = edges
                    .iter()
                    .find(|&x| {
                        (x.node1 == path[i] && x.node2 == path[i + 1])
                            || (x.node2 == path[i] && x.node1 == path[i + 1])
                    })
                    .unwrap()
                    .weight;
                // let weight = maps_for_odd_nodes.get(&path[i].clone()).unwrap().nodes.get(&path[i+1].clone()).unwrap().adj_nodes.iter().find(|&x| x.name == path[i+1]).unwrap().weight;
                // let weight = nodes.get(&path[i].clone()).unwrap().edges.iter().find(|&x| x.node2 == path[i+1]).unwrap().weight.clone();
                nodes = connect_nodes(path[i].clone(), path[i + 1].clone(), weight, nodes);
                nodes = connect_nodes(path[i + 1].clone(), path[i].clone(), weight, nodes);
            }
        }
    }

    for (key, node) in nodes.iter() {
        for edge in node.edges.iter() {
            continue;
            println!(
                "Key: {}, Destination: {}, Weight: {}",
                key, edge.node2, edge.weight
            )
        }
    }

    // find eulerian path.
    let map_result = find_eulerian_circuit(&mut nodes, "0".to_string());

    println!(
        "Distance {}, map: {}",
        map_result.weight,
        map_result
            .map
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    Ok(())
}

fn find_eulerian_circuit(nodes: &mut HashMap<String, Node>, start_node_name: String) -> MapResult {
    let mut result_map: MapResult = MapResult {
        map: Vec::new(),
        weight: 0.0,
    };

    // let mut nodes = n.clone();
    let mut current_node_name = start_node_name.clone();
    let mut should_break = false;
    println!("Current node name START: {}", current_node_name);

    for (key, node) in nodes.iter() {
        for n in node.edges.iter() {
            //println!("{}", key.eq(&"1".to_string()));
            // println!("{}", key.eq(&current_node_name));
            continue;
            println!(
                "Key: '{}', Start '{}' Destination: '{}', Weight: {}",
                key, n.node1, n.node2, n.weight
            );
        }
    }
    loop {
        let mut next_map = MapResult {
            map: Vec::new(),
            weight: 0.0,
        };
        // hierholtzer algo
        // random walk among non traversed edges to find path

        let mut next_node_name: Option<String> = None;
        let mut count = 0;
        loop {
            let mut current_node = nodes.remove(&current_node_name).unwrap();
            println!("Current node name: {}", current_node_name);
            let mut next_node_name: Option<String> = None;
            let edges_len = current_node.edges.len();
            // println!("There are {} edges to search", edges_len);
            for i in 0..edges_len {
                println!(
                    "Looking at {} -> {}",
                    current_node.edges[i].node1, current_node.edges[i].node2
                );
                // for each vert in path, check to see if edges are untravelled
                if current_node.edges[i].traversed {
                    println!(
                        "This edge {} has been traversed, continuing",
                        current_node.edges[i].node2
                    );
                    continue;
                }
                println!(
                    "This edge {} has not traversed",
                    current_node.edges[i].node2
                );
                // if they're untraveled, take them and travel until back at starting

                // mark original edge as traversed
                current_node.edges[i].traversed = true;
                next_node_name = Some(current_node.edges[i].node2.clone());
                nodes.insert(current_node_name.clone(), current_node.to_owned());

                // update the map the first time
                if next_map.map.len() == 0 {
                    println!("Updating map for the first time {}", current_node_name);
                    next_map.map.push(current_node_name.clone());
                }

                // update corresponding edge in the destination node
                let mut node_to_update_edge = nodes.remove(&next_node_name.as_deref().unwrap().to_string()).unwrap();
                let position_of_edge_to_update = node_to_update_edge
                    .edges
                    .iter()
                    .position(|x| {
                        x.node1 == current_node.edges[i].node2
                            && x.node2 == current_node.edges[i].node1
                            && x.traversed == false
                    })
                    .unwrap();
                let weight_of_edge = node_to_update_edge.edges[position_of_edge_to_update].weight;
                node_to_update_edge.edges[position_of_edge_to_update].traversed = true;
                nodes.insert(node_to_update_edge.name.clone(), node_to_update_edge);

                // update the map the first time
                if next_map.map.len() == 0 {
                    next_map.map.push(current_node_name.clone());
                } else {
                    next_map.map.push(next_node_name.as_deref().unwrap().to_string());
                }

                println!(
                    "next map is now: {}",
                    next_map
                        .map
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );

                next_map.weight = next_map.weight + weight_of_edge;
                current_node_name = next_node_name.as_deref().unwrap().to_string();
                println!(
                    "Current/next node name UPDATED: {}",
                    current_node_name.clone()
                );
                break;
            }
            match next_node_name {
                Some(ref x) => {
                    println!(
                        "Next node name: {},  start node name {}",
                        x, start_node_name
                    );   
                }
                None => {
                    println!(
                        "There is no next node name, start node name {}",
                        start_node_name
                    );
                }
            };

            // # if they're untraveled, take them and travel until back at starting
            if next_node_name.as_deref().is_none() || next_node_name.as_deref().unwrap().eq(&start_node_name)    {
                println!("NODE NAMES ARE THE SAME or we're out of nodes, BREAKING");
                break;
            }
            count = count + 1;
            if count > 500 {
                break;
            };
        }

        // merge maps
        let result_map_size = result_map.map.len();
        println!(
            "Map -> {} being added to result => {}",
            next_map
                .map
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(","),
            result_map
                .map
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        if result_map_size > 0 {
            for i in 0..(result_map.map.len() - 1) {
                println!(
                    "Checking to see if should insert at {} of {}, which is value {}",
                    i,
                    result_map.map.len(),
                    result_map.map[i]
                );
                if result_map.map[i] == next_map.map[0] {
                    let mut temp_map = result_map.map[0..i].to_vec();
                    temp_map.append(&mut next_map.map);
                    temp_map.append(&mut result_map.map[i + 1..].to_vec());
                    result_map.map = temp_map;
                    break;
                }
            }
        } else {
            result_map = next_map;
        }
        println!(
            "Map is now {}",
            result_map
                .map
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        println!("Find a new start node with state: ");
        for (key, node) in nodes.iter() {
            for n in node.edges.iter() {
                println!(
                    "Key: '{}', Start '{}' Destination: '{}', Weight: {}, Traversed: {}",
                    key, n.node1, n.node2, n.weight, n.traversed
                );
            }
        }
        let mut is_valid_node = false;
        let mut untraversed_node: Option<String> = None;
        // find a new start node
        for (key, node) in nodes.iter_mut() {
            println!("Trying {}", key);

            for e in node.edges.iter() {
                if e.traversed {
                    is_valid_node = true;
                    continue;
                }

                untraversed_node = Some(e.node1.clone());
            }
            if is_valid_node && untraversed_node.is_some() {
                // println!("Traveling to the {} node", untraversed_node.unwrap());
                break;
            }
        }
        if is_valid_node {
            match untraversed_node {
                Some(x) => {
                    println!("Setting current node name for next cycle {}", x);
                    current_node_name = x;
                }
                None => {
                    //should_break = true;
                    println!("All Done!");
                    break;
                }
            }
        }
        /* if should_break {
            println!("Breaking because should break");
            break;
        };*/
    }
    // repeat
    // splice the map in
    println!("End state");
    for (key, node) in nodes.iter() {
        for n in node.edges.iter() {
            println!(
                "Key: '{}', Start '{}' Destination: '{}', Weight: {}, Traversed: {}",
                key, n.node1, n.node2, n.weight, n.traversed
            );
        }
    }

    return result_map;
}

fn get_all_pair_combinations(p: &Vec<Pair>, map: &HashMap<String, DjikstraNodes>) -> Vec<Vec<Pair>> {
    println!("Getting all pair combinations");
    let mut pairs = p.clone();
    let mut results: Vec<Vec<Pair>> = Vec::new();

    let mut distance_threshold = 0.5;

    loop {
        for pair in pairs.iter() {
            println!(
                "#### getting pair combinations starting with {} {}, out of {} pairs",
                pair.node1, pair.node2, pairs.len()
            );
            
            match get_pair_combinations(p, pair.clone(), map,  "".to_string(), distance_threshold ) {
                Some(combinations) => {
                    
                    let mut should_insert_combination: bool = true;
                    for result in results.iter() {
                        // println!("I'm here");
                        if are_sets_of_pairs_eqivilent(result, &combinations) {
                            should_insert_combination = false;
                            break;
                        }
                    }
                    if should_insert_combination {
                        for combination in combinations.iter() {
                            print!("{}-{}, ", combination.node1, combination.node2)
                        }
                        println!("");
                        results.push(combinations);
                    }
                },
                None => {

                }
            };

            /*let combinations: Vec<Pair> =
                get_pair_combinations(p, /*nodes.clone(), */ pair.clone(), "".to_string());

            let mut should_insert_combination: bool = true;
            for result in results.iter() {
                // println!("I'm here");
                if are_sets_of_pairs_eqivilent(result, &combinations) {
                    should_insert_combination = false;
                    break;
                }
            }
            if should_insert_combination {
                for combination in combinations.iter() {
                    print!("{}-{}, ", combination.node1, combination.node2)
                }
                println!("");
                results.push(combinations);
            } */
        }
        if results.len() > 1 {break;}
        distance_threshold = distance_threshold + 0.1;
    }

    return results;
}

fn are_sets_of_pairs_eqivilent(one_pairs: &Vec<Pair>, two_pairs: &Vec<Pair>) -> bool {
    let result: bool = false;
    if one_pairs.len() != two_pairs.len() {
        return false;
    }
    let mut count = 0;
    for one in one_pairs {
        let mut found: bool = false;
        for two in two_pairs {
            /*if count % 10 == 0 {
                println!("I'm on pair 1 {}, and pair 2 {}", one.node1, two.node1);
            }*/
            if (one.node1 == two.node1 && one.node2 == two.node2)
                || (one.node1 == two.node2 && one.node2 == two.node1)
            {
                found = true;
            }
            count = count + 1;
        }
        if !found {
            return false;
        }
    }

    return true;
}

fn get_pair_combinations(
    p: &Vec<Pair>,
    // mut n: Vec<String>,
    selected_start_pair: Pair,
    map: &HashMap<String, DjikstraNodes>,
    recurse_depth: String,
    distance_threshold: f32,
) -> Option<Vec<Pair>> {
    let mut result: Vec<Pair> = Vec::new();
    let mut pairs = p.clone();
    
    // println!("Start nodes are {} \n", nodes.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));

            // println!("Found a start pair");

            // println!("Checking pair {} {}, with nodes left {} \n",selected_start_pair.node1, selected_start_pair.node2, nodes.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));


                // add the pair
                result.push(selected_start_pair.clone());
                // remove the nodes from the acceptable list
                //nodes.retain(|x| !x.eq(&selected_start_pair.node1));
                //nodes.retain(|x| !x.eq(&selected_start_pair.node2));
                // println!("There were {} pairs before", pairs.len());
                pairs.retain(|x| {
                    !((x.node1.eq(&selected_start_pair.node1)
                        || x.node2.eq(&selected_start_pair.node2))
                        || (x.node1.eq(&selected_start_pair.node2)
                            || x.node2.eq(&selected_start_pair.node1)))
                });
                //println!("There were {} pairs after", pairs.len());

                // println!("Pushing pair {} {}, now nodes left are {} \n", selected_start_pair.node1, selected_start_pair.node2, nodes.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));
                if pairs.len() > 0 {
                    //
                    // println!("nodes left are {} \n", nodes.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));
                    let mut count = 0;
                    // let start = SystemTime::now();
                    // println!("count before {}", count);
                

                        
                    for pair in pairs.iter() {
                        let mut new_recurse_depth: String = recurse_depth.clone();
                        new_recurse_depth.push_str("#");
                        new_recurse_depth.push_str(&count.to_string());
                        //println!("{}", new_recurse_depth);

                        //count = count + 1;
                        //println!("count druing {}", count);
                        //let since_the_epoch = start
                        //.duration_since(UNIX_EPOCH)
                        // .expect("Time went backwards");
                        //let in_ms = since_the_epoch.as_secs() * 1000 +
                        //  since_the_epoch.subsec_nanos() as u64 / 1_000_000;
                        if  rand::thread_rng().gen_range(0..100000) == 42 {
                            //} && since_the_epoch.as_secs() % 2 == 0 {
                            println!(
                                "{} recursing {} out of {} times.",
                                new_recurse_depth, 
                                count,
                                pairs.len()
                            );
                        }
                        count = count + 1;
                        println!("Pair has td: {}, compared to tmd: {}, with factor: {}",  map.get(&pair.node1).unwrap().nodes.get(&pair.node2).unwrap().total_distance as f32, map.get(&pair.node1).unwrap().median_total_distance as f32, distance_threshold );
                        if map.get(&pair.node1).unwrap().nodes.get(&pair.node2).unwrap().total_distance as f32 > map.get(&pair.node1).unwrap().median_total_distance as f32 * distance_threshold {
                            return None
                        }

                        match get_pair_combinations(
                            &pairs,
                            // nodes.clone(),
                            pair.clone(),
                            map,
                            new_recurse_depth.clone(),
                            distance_threshold,
                        ) {
                            Some(mut x) => {
                                result.append(&mut x);
                            },
                            None => {
                                return None;
                            }
                        }
                        /*
                        result.append(&mut get_pair_combinations(
                            &pairs,
                            // nodes.clone(),
                            pair.clone(),
                            new_recurse_depth.clone(),
                            distance_threshold,
                        ).unwrap());
                        */
                    }
                    //println!("count after {}", count);
                }

    // println!("####Done recursing!\n");
    return Some(result);
}

fn  get_pairs(n: &Vec<String>) -> Vec<Pair> {
    if n.len() % 2 != 0 {
        panic!("There should never be an odd number of pair options");
    }
/*
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

        if insert_node1 {
            nodes.push(pair.node1.clone());
            println!("{}", pair.node1.clone());
        }
        if insert_node2 {
            nodes.push(pair.node2.clone());
            println!("{}", pair.node2.clone());
        }
    }*/

    let mut names = n.clone();
    let mut results: Vec<Pair> = Vec::new();

    for node1 in names.iter() {
        // println!("looking for pairs for {}", node1);
        for node2 in names.iter() {
            // println!("## does that pair with {}", node2);
            if node1.eq(node2) {
                continue;
            }
            let mut should_insert: bool = true;
            let mut p = Pair {
                node1: node1.clone(),
                node2: node2.clone(),
                distance: None,
                path: Vec::new(),
            };

            for result in &results {
                // filter out equivilent dupes
                // println!("### Compairing result {}, {}, and pair {}, {} ", result.node1, result.node2, p.node1, p.node2);
                if (result.node1.eq(node1) && result.node2.eq(node2))
                    || (result.node1.eq(node2) && result.node2.eq(node1))
                {
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

    return results;
}

fn get_pairs_distance(p: &Vec<Pair>, map: &HashMap<String, DjikstraNodes> ) -> Vec<Pair> {
    println!("Getting distance between the pairs");
    // let mut pairs = p.clone();
    let mut results: Vec<Pair> = Vec::new();

    // let mut max_distance = 1;

    for pair in p.iter() {

       let node = map
                .get(&pair.node1)
                .unwrap()
                .nodes
                .get(&pair.node2)
                .unwrap();

        let mut pair_with_distance = pair.clone();
        pair_with_distance.distance = Some(node.total_distance);
        pair_with_distance.path = node.path.clone();

        results.push(pair_with_distance.clone());
    }

    return results;
}

struct NodeTraveled {
    node: String,

}

// get the N smallest pairs for each 
fn get_optimized_pairs(p: &Vec<Pair>) -> Vec<Pair> {

    let mut nodes: Vec<String> = Vec::new();

    for pair in p.iter() {
        let mut insert_node1: bool = true;
        let mut insert_node2: bool = true;
        if nodes.iter().any(|i| i.eq(&pair.node1)) {
            insert_node1 = false;
        }

        if nodes.iter().any(|i| i.eq(&pair.node2)) {
            insert_node2 = false;
        }

        if insert_node1 {
            nodes.push(pair.node1.clone());
            println!("{}", pair.node1.clone());
        }
        if insert_node2 {
            nodes.push(pair.node2.clone());
            println!("{}", pair.node2.clone());
        }
    }

    let mut min_distance = 1;
    let mut results: Vec<Pair> = Vec::new();
    loop {
    
        let mut done_nodes: Vec<String> = Vec::new(); //nodes.clone();


        results = Vec::new();
        println!("Min distance: {}", min_distance);
        for rn in done_nodes.iter() {
                
                print!(" {} ", rn);
                println!("");
        }

        for pair in p.iter() {

            if pair.path.len() == (min_distance + 1) { // plus 1 is because the shortest path from A->B is A,B, of len 2;
                done_nodes.push(pair.node1.clone());
                done_nodes.push(pair.node2.clone());
                //remaining_nodes.retain(|x| x.as_str() != pair.node1 || x.as_str() != pair.node2);

                results.push(pair.clone());
            }
        }

        done_nodes.sort_unstable();
        done_nodes.dedup();
        // if we've found all the nodes, we're done, get out of here. 
        if done_nodes.len() == nodes.len() {break;}

        if min_distance > 15 {
            // this is basically a loop stopper, obviously not valid for some graphs, but we should be able to get to everything in our map without doing 15 hops lol. 
            // TODO: make this based off the longest path in the node maps. 
            panic!("Something has gone wrong, it should never take this many hops to find all the nodes, something is busted somewhere.")
        }
        min_distance = min_distance + 1;
        
    }

    return results;
}

fn is_pair_inside_distance(pair: &Pair, distance: i32, n: &HashMap<String, Node> ) -> bool {

    if distance < 1 { panic!("Bad distance");};
    let mut start_node = n.get(&pair.node1).unwrap();

    for edge in start_node.edges.iter() {
        if (edge.node1 == pair.node1 && edge.node2 == pair.node2) || (edge.node1 == pair.node2 && edge.node2 == pair.node1) {
            return true;
        }

        if distance > 1 { panic! ("we haven't build that yet");}
    }
    
    return false;
}


#[derive(Clone)]
struct Pair {
    node1: String,
    node2: String,
    distance: Option<f32>,
    path: Vec<String>, 
}

#[derive(Clone)]
struct DjikstraNode {
    name: String,
    total_distance: f32,
    path: Vec<String>,
    traversed: bool,
    adj_nodes: Vec<NodeWeightMap>,
}

struct DjikstraNodes {
    start_node: String,
    nodes: HashMap<String, DjikstraNode>,
    median_total_distance: f32
}

#[derive(Clone)]
struct NodeWeightMap {
    name: String,
    weight: f32,
}

fn find_median_distance(start: &String, graph: &mut DjikstraNodes) -> () {
    let mut distances: Vec<f32> = Vec::new();
    for node in graph.nodes.get(&graph.start_node).unwrap().adj_nodes.iter() {
        println!("looking at node {} with td: {} ", node.name, node.weight);
        distances.push(node.weight);
    }
    //println!("distances: {}", distances.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
    distances.sort_by(|a, b| a.partial_cmp(b).unwrap()); 
    println!("distances: {}", distances.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
    let median_distance = distances[distances.len() /2];
    println!("The mtd for {}: {}", start, median_distance);
    graph.median_total_distance = median_distance;
    //panic!("ben");
}

fn find_shortest_path(start: &String, graph: &mut DjikstraNodes) -> () {
    // Done in earlier function: for nodes, mark node dist(0), rest dist(infinity)

    //println!("There are {} DjikstraNodes", graph.nodes.keys().len());

    if start.eq(&graph.start_node) {
        let mut node_being_looked_at: DjikstraNode = graph.nodes.remove(start).unwrap();
        node_being_looked_at.total_distance = 0.0;
        node_being_looked_at.path.push(start.clone());
        //println!("Start {} and start node {} are equal. Path: {}", start, &graph.start_node, node_being_looked_at.path.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
        graph.nodes.insert(start.clone(), node_being_looked_at);
    }

    let current_node_name = start;
    //println!("Starting search on node {}", current_node_name);

    let mut current_node = graph.nodes.remove(current_node_name).unwrap();

    let mut name_of_closest_node: String = "".to_string();
    let mut distance_of_closest_node: f32 = f32::MAX;
    for node in current_node.adj_nodes.iter() {
        if graph.nodes.get(&node.name).unwrap().traversed {
            //println!("Node {} has already been traveled", &node.name);
            continue;
        }
        let mut node_being_looked_at: DjikstraNode = graph.nodes.remove(&node.name).unwrap();
        // println!("Looking at node {}", node_being_looked_at.name);
        // for node update edges with total distance from node if distance less than existing distance
        // update nodes edges with map from node -> neighbors
        if node.weight + current_node.total_distance < node_being_looked_at.total_distance {
            node_being_looked_at.total_distance = node.weight + current_node.total_distance;
            if node_being_looked_at.path.len() == 0 {
                for p in current_node.path.iter() {
                    node_being_looked_at.path.push(p.clone());
                }
                node_being_looked_at.path.push(node.name.clone());
                //println!("Node being looked at path was empty, is now {}", node_being_looked_at.path.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
            } else {
                node_being_looked_at.path = current_node.path.clone();
                node_being_looked_at.path.push(node.name.clone());
                //println!("Node being looked at path was not empty, is now {}", node_being_looked_at.path.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
            }
        } else {
            //println!("node weight {} was not less than node being looked at total distance {}", node.weight, node_being_looked_at.total_distance);
        }

        graph
            .nodes
            .insert(node.name.to_string(), node_being_looked_at);

        if !graph.nodes.get(&node.name).unwrap().traversed
            && graph.nodes.get(&node.name).unwrap().total_distance < distance_of_closest_node
        {
            distance_of_closest_node = graph.nodes.get(&node.name).unwrap().total_distance;
            name_of_closest_node = node.name.clone();
        }
    }

    current_node.traversed = true;
    graph
        .nodes
        .insert(current_node_name.to_string(), current_node);

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
    for record in graph.iter() {
        if !nodes.contains(&record.node1) {
            nodes.push(record.node1.clone())
        }

        if !nodes.contains(&record.node2) {
            nodes.push(record.node2.clone())
        }
    }

    for node in nodes.iter() {
        let mut count = 0;

        for record in graph.iter() {
            if &record.node1 == node || &record.node2 == node {
                count = count + 1;
            }
        }

        if count % 2 == 1 {
            odd_count = odd_count + 1;
            odd_nodes.push(node.clone());
        }
    }

    // traversable only if 2 or 0 because math
    if odd_count == 0 {
        return None;
    }

    return Some(odd_nodes);
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

        println!("Nodes: {},{}, Weight: {} ", e.node1, e.node2, e.weight,);
        g.push(e);
    }
    return g;
}

fn map_nodes(data: &Vec<Record>) -> HashMap<String, Node> {
    let mut g: HashMap<String, Node> = HashMap::new();
    for record in data {
        // undirectional, so you have to do both.
        g = connect_nodes(
            record.node1.trim().to_string(),
            record.node2.trim().to_string(),
            record.weight,
            g,
        );
        g = connect_nodes(
            record.node2.trim().to_string(),
            record.node1.trim().to_string(),
            record.weight,
            g,
        );
    }
    return g;
}

fn map_to_djikstra_nodes(data: &Vec<Record>) -> DjikstraNodes {
    let mut g: DjikstraNodes = DjikstraNodes {
        start_node: "".to_string(),
        nodes: HashMap::new(),
        median_total_distance: 0.0
    };

    for record in data {
        // undirectional, so you have to do both.
        g = connect_djikstra_nodes(
            record.node1.trim().to_string(),
            record.node2.trim().to_string(),
            record.weight,
            g,
        );
        g = connect_djikstra_nodes(
            record.node2.trim().to_string(),
            record.node1.trim().to_string(),
            record.weight,
            g,
        );
    }
    return g;
}

fn connect_nodes(
    start: String,
    end: String,
    weight: f32,
    g: HashMap<String, Node>,
) -> HashMap<String, Node> {
    let mut graph = g;
    if graph.contains_key(&start) {
        // println!("Found Key: {}", &start);
        let e = Edge {
            node1: start.clone(),
            node2: end.clone(),
            weight: weight,
            traversed: false,
        };
        graph.get_mut(&start).unwrap().edges.push(e);
    } else {
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

    return graph;
}

fn connect_djikstra_nodes(
    start: String,
    end: String,
    weight: f32,
    g: DjikstraNodes,
) -> DjikstraNodes {
    let mut graph = g;
    if graph.nodes.contains_key(&start) {
        // println!("Found Key: {}", &start);
        let e = NodeWeightMap {
            name: end.clone(),
            weight: weight,
        };
        graph.nodes.get_mut(&start).unwrap().adj_nodes.push(e);
    } else {
        // println!("No Key: {}", &start);
        let e = NodeWeightMap {
            name: end.clone(),
            weight: weight.clone(),
        };
        let mut n = DjikstraNode {
            total_distance: f32::MAX,
            path: Vec::new(),
            adj_nodes: Vec::new(),
            traversed: false,
            name: start.clone(),
        };
        n.adj_nodes.push(e);
        graph.nodes.insert(start.clone(), n);
    }

    return graph;
}

fn print_graphml(records: &Vec<Record>) {

    let mut nodes: Vec<String> = Vec::new();

    for record in records.iter() {
        let mut insert_node1: bool = true;
        let mut insert_node2: bool = true;
        if nodes.iter().any(|i| i.eq(&record.node1.trim())) {
            insert_node1 = false;
        }

        if nodes.iter().any(|i| i.eq(&record.node2.trim())) {
            insert_node2 = false;
        }

        if insert_node1 {
            nodes.push(record.node1.clone().trim().to_string());
            //println!("{}", record.node1.clone());
        }
        if insert_node2 {
            nodes.push(record.node2.clone().trim().to_string());
            //println!("{}", record.node2.clone());
        }
    }

    for node in nodes.iter() {
       // println!("<node id=\"{}\"/>", node.trim());
       println!("{}", node.trim());
    }

    for record in records.iter() {
        let mut owned_string: String = record.node1.trim().to_owned();
        let another_owned_string: String = record.node2.trim().to_owned();
        
        owned_string.push_str("-");
        owned_string.push_str(&another_owned_string);
        /*println!("<edge id=\"{}\" source=\"{}\" target=\"{}\">
        <data key=\"d1\">{}</data>
      </edge>", owned_string, record.node1.trim(), record.node2.trim(), record.weight);*/
      println!("{} {} {}", record.node1.trim(), record.node2.trim(), record.weight);
    }
    panic!("done printing graphml");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_optimized_pairs_1_hop() {
        let mut start: Vec<Pair> = Vec::new();
        start.push(Pair {node1: "A".to_string(), node2: "B".to_string(), distance: Some(5.0), path: vec!["A".to_string(),"B".to_string()]});
        start.push(Pair {node1: "A".to_string(), node2: "C".to_string(), distance: Some(5.0), path: vec!["A".to_string(),"C".to_string()]});
        start.push(Pair {node1: "A".to_string(), node2: "D".to_string(), distance: Some(5.0), path: vec!["A".to_string(),"B".to_string(), "D".to_string()]});
        start.push(Pair {node1: "B".to_string(), node2: "C".to_string(), distance: Some(5.0), path: vec!["B".to_string(), "A".to_string(), "C".to_string()]});
        start.push(Pair {node1: "B".to_string(), node2: "D".to_string(), distance: Some(5.0), path: vec!["B".to_string(),"D".to_string()]});
        start.push(Pair {node1: "C".to_string(), node2: "D".to_string(), distance: Some(5.0), path: vec!["C".to_string(),"D".to_string()]});
    
       let results = get_optimized_pairs(&start);
       for result in results.iter() {
           println!("{}, {}, {}", result.node1, result.node2,              result.path.iter()
           .map(|x| x.to_string())
           .collect::<Vec<_>>()
           .join(","))
       }
       assert_eq!(results.len(), 4);
       assert_eq!(results[0].node1, "A");
       assert_eq!(results[0].node2, "B");

       assert_eq!(results[1].node1, "A");
       assert_eq!(results[1].node2, "C");

       assert_eq!(results[2].node1, "B");
       assert_eq!(results[2].node2, "D");

       assert_eq!(results[3].node1, "C");
       assert_eq!(results[3].node2, "D");
    }

    #[test]
    fn test_get_optimized_pairs_2_hop() {
        let mut start: Vec<Pair> = Vec::new();
        start.push(Pair {node1: "A".to_string(), node2: "B".to_string(), distance: Some(5.0), path: vec!["A".to_string(),"B".to_string()]});
        start.push(Pair {node1: "A".to_string(), node2: "C".to_string(), distance: Some(5.0), path: vec!["A".to_string(),"C".to_string()]});
        start.push(Pair {node1: "A".to_string(), node2: "D".to_string(), distance: Some(5.0), path: vec!["A".to_string(),"B".to_string(), "D".to_string()]});
        start.push(Pair {node1: "B".to_string(), node2: "C".to_string(), distance: Some(5.0), path: vec!["B".to_string(), "A".to_string(), "C".to_string()]});
        start.push(Pair {node1: "B".to_string(), node2: "D".to_string(), distance: Some(5.0), path: vec!["B".to_string(),"D".to_string()]});
        start.push(Pair {node1: "C".to_string(), node2: "D".to_string(), distance: Some(5.0), path: vec!["C".to_string(),"D".to_string()]});
    
       let results = get_optimized_pairs(&start);
       for result in results.iter() {
           println!("{}, {}, {}", result.node1, result.node2,              result.path.iter()
           .map(|x| x.to_string())
           .collect::<Vec<_>>()
           .join(","))
       }
       assert_eq!(results.len(), 4);
       assert_eq!(results[0].node1, "A");
       assert_eq!(results[0].node2, "B");

       assert_eq!(results[1].node1, "A");
       assert_eq!(results[1].node2, "C");

       assert_eq!(results[2].node1, "B");
       assert_eq!(results[2].node2, "D");

       assert_eq!(results[3].node1, "C");
       assert_eq!(results[3].node2, "D");
    }
}