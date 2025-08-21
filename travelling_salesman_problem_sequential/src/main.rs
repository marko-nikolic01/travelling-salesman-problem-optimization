use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn enumerate_cities(file_path: &str, cities: Vec<String>) -> io::Result<(HashMap<String, usize>, Vec<String>)> {
    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = io::BufReader::new(file);

    let mut city_to_id: HashMap<String, usize> = HashMap::new();
    let mut id_to_city: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line: String = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 5 { continue; }

        let origin: String = format!("{}, {}", parts[0].trim(), parts[1].trim());
        let destination: String = format!("{}, {}", parts[2].trim(), parts[3].trim());

        for city in [origin, destination] {
            let city_name: &str = city.split(',').next().unwrap().trim();

            if city_to_id.contains_key(&city) { continue; }

            if cities.contains(&city_name.to_string()) {
                let id: usize = id_to_city.len();
                city_to_id.insert(city.clone(), id);
                id_to_city.push(city);
            }
        }
    }

    Ok((city_to_id, id_to_city))
}

fn build_city_adjacency_list(file_path: &str, city_to_id: &HashMap<String, usize>) -> io::Result<Vec<Vec<i32>>> {
    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = io::BufReader::new(file);

    let n: usize = city_to_id.len();
    let mut city_graph: Vec<Vec<i32>> = vec![vec![0; n]; n];

    for line in reader.lines() {
        let line: String = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 5 { continue; }

        let origin: String = format!("{}, {}", parts[0].trim(), parts[1].trim());
        let destination: String = format!("{}, {}", parts[2].trim(), parts[3].trim());
        let distance: i32 = parts[4].trim().parse().unwrap_or(0);

        if let (Some(&origin_id), Some(&destination_id)) = (city_to_id.get(&origin), city_to_id.get(&destination)) {
            city_graph[origin_id][destination_id] = distance;
            city_graph[destination_id][origin_id] = distance;
        }
    }

    Ok(city_graph)
}

fn main() {
    let cities: Vec<String> = env::args().skip(1).collect();
    let n: usize = cities.len();
    if n < 2 {
        eprintln!("Error: You must pass a list of cities - cargo run <city1> <city2> ... <cityN>");
        return;
    }

    let csv_file: &'static str = "../input.txt";
    let (city_to_id, id_to_city) = enumerate_cities(&csv_file, cities).unwrap();
    let city_graph:Vec<Vec<i32>> = build_city_adjacency_list(&csv_file, &city_to_id).unwrap();

    println!("Adjacency list (city distances):");
    for (i, row) in city_graph.iter().enumerate() {
        print!("{} ({}): ", i, id_to_city[i]);
        for &dist in row {
            if dist == i32::MAX {
                print!("INF ");
            } else {
                print!("{} ", dist);
            }
        }
        println!();
    }

    let possible_states: usize = 1 << n;
    let mut shortest_paths: Vec<i32> = vec![i32::MAX; possible_states];
    let mut last_city: Vec<i32> = vec![-1; possible_states];
    let mut previous_city: Vec<i32> = vec![-1; possible_states];

    shortest_paths[0] = 0;
    last_city[0] = 0;

    let mut state_traversal: Vec<i32> = vec![0];
    let mut next_state_traversal: Vec<i32>;

    let first_city: usize = 0;
    let mut city: i32;
    let mut next_state: i32;
    let mut next_distance: i32;
    let mut is_city_visited: bool;
    let mut is_first_city: bool;
    let mut can_visit_first_city: bool;
    let mut is_state_unvisited: bool;
    let mut is_new_distance_shorter:bool;
    
    for iteration in 0..n {
        next_state_traversal = Vec::new();

        can_visit_first_city = iteration == n - 1;

        for &state in &state_traversal {
            for city_index in 0..n {
                city = 1 << city_index;

                is_city_visited = city & state != 0;
                is_first_city = city_index == first_city;

                if !is_city_visited && (!is_first_city || can_visit_first_city) {
                    next_state = state | city;
                    next_distance = shortest_paths[state as usize] + city_graph[last_city[state as usize] as usize][city_index];

                    is_state_unvisited = last_city[next_state as usize] == -1;
                    is_new_distance_shorter = next_distance < shortest_paths[next_state as usize];

                    if is_state_unvisited || is_new_distance_shorter {
                        shortest_paths[next_state as usize] = next_distance;
                        last_city[next_state as usize] = city_index as i32;
                        previous_city[next_state as usize] = last_city[state as usize];
                    }

                    if is_state_unvisited {
                        next_state_traversal.push(next_state);
                    }
                }
            }
        }
    
        state_traversal = next_state_traversal;
    }

    let final_state: usize = possible_states - 1;
    println!("Shortest TSP distance: {}", shortest_paths[final_state]);
}
