use std::time::Instant;

mod info;
mod structs;

use info::*;
use structs::*;

use info::read_csv_to_hashmap;

fn main() {
    let rnvot_map = create_region_name_to_vector_of_teams_map();
    let rnr_map = create_region_name_to_region_map(&rnvot_map);

    let now = Instant::now();

    for _ in 0..1000 {
        let bracket = Bracket::new(
            rnr_map.get("south").unwrap().clone(),
            rnr_map.get("east").unwrap().clone(),
            rnr_map.get("west").unwrap().clone(),
            rnr_map.get("midwest").unwrap().clone(),
        );

        let br = bracket.sample_result();

        br.write_to_csv("sim_results.csv");
    }

    let t_e = now.elapsed();

    println!("time to run and save 1000 sims: {:?}", t_e);
}
