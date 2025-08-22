use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
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
    let n: usize = city_graph.len();
    let possible_states: usize = 1 << n;

    let shortest_paths: Arc<Vec<Vec<Mutex<i32>>>> = Arc::new(
        (0..possible_states)
            .map(|_| (0..n).map(|_| Mutex::new(i32::MAX)).collect())
            .collect()
    );
    let previous_cities: Arc<Vec<Vec<Mutex<i32>>>> = Arc::new(
        (0..possible_states)
            .map(|_| (0..n).map(|_| Mutex::new(-1)).collect())
            .collect()
    );
    let is_state_used: Arc<Mutex<Vec<bool>>> = Arc::new(Mutex::new(vec![false; possible_states]));

    let mut states: Vec<i32> = (1..n).map(|i| {
        let state = 1 << i;
        *shortest_paths[state][i].lock().unwrap() = city_graph[first_city][i];
        *previous_cities[state][i].lock().unwrap() = first_city as i32;
        is_state_used.lock().unwrap()[state] = true;
        state as i32
    }).collect();

    let mut can_visit_first_city: bool;
    let mut next_states_mutex: Arc<Mutex<Vec<i32>>>;

    for iteration in 1..n {
        can_visit_first_city = iteration == n - 1;
        next_states_mutex = Arc::new(Mutex::new(Vec::new()));

        states.par_iter().for_each(|&state| {
            let mut next_states = Vec::new();

            for city_index in 0..n {
                let city_bit = 1 << city_index;
                if (city_bit & state != 0) || (city_index == first_city && !can_visit_first_city) {
                    continue;
                }

                let next_state = state | city_bit;

                {
                    let mut used = is_state_used.lock().unwrap();
                    if !used[next_state as usize] {
                        next_states.push(next_state);
                        used[next_state as usize] = true;
                    }
                }

                for prev_index in 0..n {
                    let prev_bit = 1 << prev_index;
                    if prev_bit & state == 0 { continue; }

                    let mut sp_cell = shortest_paths[next_state as usize][city_index].lock().unwrap();
                    let sp_prev = shortest_paths[state as usize][prev_index].lock().unwrap();
                    let new_distance = *sp_prev + city_graph[prev_index][city_index];

                    if new_distance < *sp_cell {
                        *sp_cell = new_distance;
                        *previous_cities[next_state as usize][city_index].lock().unwrap() = prev_index as i32;
                    }
                }
            }

            let mut next_states_lock = next_states_mutex.lock().unwrap();
            next_states_lock.extend(next_states);
        });

        states = next_states_mutex.lock().unwrap().to_vec();
    }

    let shortest_distance = *shortest_paths[possible_states - 1][0].lock().unwrap();

    let mut path = Vec::new();
    let mut state = possible_states - 1;
    let mut city = first_city;

    while *previous_cities[state][city].lock().unwrap() > -1 {
        path.push(city);
        let prev_city = *previous_cities[state][city].lock().unwrap() as usize;
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

    let output_file = format!("resources/output/output_parallel_{}.txt", n);
    save_solution(duration, shortest_distance, &path, &city_graph, &id_to_city, &output_file).unwrap();
}
