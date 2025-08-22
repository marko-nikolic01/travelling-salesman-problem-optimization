use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::time::{Instant, Duration};

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

fn save_solution(duration: Duration, shortest_distance: i32, path: &Vec<usize>, city_graph: &Vec<Vec<i32>>, id_to_city: &Vec<String>, output_file: &str) -> io::Result<()> {
    let mut file = File::create(output_file)?;

    writeln!(file, "{:.6}", duration.as_secs_f64())?;
    writeln!(file, "{}", shortest_distance)?;

    for i in 0..path.len() - 1 {
        let from = path[i];
        let to = path[i + 1];
        let dist = city_graph[from][to];
        let parts_from: Vec<&str> = id_to_city[from].split(',').map(|s| s.trim()).collect();
        let parts_to: Vec<&str> = id_to_city[to].split(',').map(|s| s.trim()).collect();
        writeln!(file, "{},{},{},{},{}", parts_from[0], parts_from[1], parts_to[0], parts_to[1], dist)?;
    }

    Ok(())
}

fn find_shortest_path(city_graph: &Vec<Vec<i32>>, first_city: usize) -> (i32, Vec<usize>) {
    let n = city_graph.len();
    let possible_states = 1 << n;

    let mut shortest_paths: Vec<Vec<i32>> = vec![vec![i32::MAX; n]; possible_states];
    let mut previous_cities: Vec<Vec<i32>> = vec![vec![-1; n]; possible_states];
    let mut is_state_used = vec![false; possible_states];


    let mut states = vec![];
    let mut next_states: Vec<i32>;

    let mut next_state: i32; 
    let mut next_distance: i32;
    let mut city: i32;
    let mut previous_city: i32;
    let mut can_visit_first_city: bool;
    let mut is_city_visited: bool;
    let mut is_first_city: bool;

    for i in 1..n {
        let state = 1  << i;

        shortest_paths[state][i] = city_graph[first_city][i];
        previous_cities[state][i] = first_city as i32;
        is_state_used[state] = true;

        states.push(state as i32);
    }

    for iteration in 1..n {
        next_states = vec![];
        can_visit_first_city = iteration == n - 1;

        for &state in &states {
            for city_index in 0..n {
                city = 1 << city_index;

                is_city_visited = city & state != 0;
                is_first_city = city_index == first_city;
                if is_city_visited || (is_first_city && !can_visit_first_city) { continue; }

                next_state = state | city;

                if !is_state_used[next_state as usize] {
                    next_states.push(next_state);
                    is_state_used[next_state as usize] = true;
                }

                for previous_city_index in 0..n {
                    previous_city = 1 << previous_city_index;

                    is_city_visited = previous_city & state != 0;
                    if !is_city_visited { continue; }

                    next_distance = shortest_paths[state as usize][previous_city_index] + city_graph[previous_city_index][city_index];

                    if next_distance < shortest_paths[next_state as usize][city_index] {
                        shortest_paths[next_state as usize][city_index] = next_distance;
                        previous_cities[next_state as usize][city_index] = previous_city_index as i32;
                    }
                }
            }
        }

        states = next_states;
    }

    let shortest_distance = shortest_paths[possible_states - 1][0];

    let mut path = Vec::new();
    let mut state = possible_states - 1;
    let mut city = first_city;

    while previous_cities[state][city] > -1 {
        path.push(city);
        let prev_city = previous_cities[state][city] as usize;
        state ^= 1 << city;
        city = prev_city;
    }
    
    path.push(first_city);
    path.reverse();

    (shortest_distance, path)
}

fn main() {
    let cities: Vec<String> = env::args().skip(1).collect();
    let n: usize = cities.len();
    if n < 2 {
        eprintln!("Error: You must pass a list of cities - cargo run <city1> <city2> ... <cityN>");
        return;
    }

    let csv_file: &'static str = "resources/input/input.txt";
    let (city_to_id, id_to_city) = enumerate_cities(&csv_file, cities).unwrap();
    let city_graph: Vec<Vec<i32>> = build_city_adjacency_list(&csv_file, &city_to_id).unwrap();

    let first_city: usize = 0;

    let start: Instant = Instant::now();
    let (shortest_distance, path) = find_shortest_path(&city_graph, first_city);
    let duration: Duration = start.elapsed();

    let output_file = format!("resources/output/output_sequential_{}.txt", n);
    save_solution(duration, shortest_distance, &path, &city_graph, &id_to_city, &output_file).unwrap();
}
