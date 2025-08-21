use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn enumerate_cities(file_path: &str, cities: Vec<String>) -> io::Result<(HashMap<String, usize>, Vec<String>)> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut city_to_id = HashMap::new();
    let mut id_to_city = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 5 { continue; }

        let origin = format!("{}, {}", parts[0].trim(), parts[1].trim());
        let destination = format!("{}, {}", parts[2].trim(), parts[3].trim());

        for city in [origin, destination] {
            let city_name = city.split(',').next().unwrap().trim();

            if city_to_id.contains_key(&city) { continue; }

            if cities.contains(&city_name.to_string()) {
                let id = id_to_city.len();
                city_to_id.insert(city.clone(), id);
                id_to_city.push(city);
            }
        }
    }

    Ok((city_to_id, id_to_city))
}

fn build_city_adjacency_list(file_path: &str, city_to_id: &HashMap<String, usize>) -> io::Result<Vec<Vec<(usize, usize)>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let n = city_to_id.len();
    let mut city_graph: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 5 { continue; }

        let origin = format!("{}, {}", parts[0].trim(), parts[1].trim());
        let destination = format!("{}, {}", parts[2].trim(), parts[3].trim());
        let distance: usize = parts[4].trim().parse().unwrap_or(0);

        if let (Some(&origin_id), Some(&destination_id)) = (city_to_id.get(&origin), city_to_id.get(&destination)) {
            city_graph[origin_id].push((destination_id, distance));
            city_graph[destination_id].push((origin_id, distance));
        }
    }

    Ok(city_graph)
}

fn main() {
    let cities: Vec<String> = env::args().skip(1).collect();
    let n = cities.len();
    if n < 2 {
        eprintln!("Error: You must pass a list of cities - cargo run <city1> <city2> ... <cityN>");
        return;
    }

    let csv_file = "../input.txt";
    let (city_to_id, id_to_city) = enumerate_cities(&csv_file, cities).unwrap();
    let city_graph = build_city_adjacency_list(&csv_file, &city_to_id).unwrap();

    let possible_states = 1 << n;
    let mut shortest_paths = vec![usize::MAX; possible_states];
    let mut previous_cities = vec![-1; possible_states];
    let mut state_traversal = vec![0];

    let mut state_traversal = vec![0];
    let first_city = 0;

    for i in 0..n {
        
    }
}
