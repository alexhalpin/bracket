use csv::ReaderBuilder;
use std::collections::HashMap;
use std::error::Error;

use crate::structs::{Region, Team};

pub fn read_csv_to_hashmap(file_path: &str) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;

    let mut map = HashMap::new();

    // Iterate through the CSV records and insert them into the HashMap
    for result in rdr.records() {
        let record = result?;

        // Assume two columns in each row
        if record.len() >= 2 {
            let key = record[0].to_string();

            if let Ok(value) = record[1].parse::<f64>() {
                map.insert(key, value);
            } else {
                eprintln!("Error parsing value for key: {}", key); // Handle the error as needed
            }
        }
    }

    Ok(map)
}

pub const south_team_names: [&'static str; 16] = [
    "Auburn Tigers",
    "Michigan State Spartans",
    "Iowa State Cyclones",
    "Texas A&M Aggies",
    "Michigan Wolverines",
    "Ole Miss Rebels",
    "Marquette Golden Eagles",
    "Louisville Cardinals",
    "Creighton Bluejays",
    "New Mexico Lobos",
    "North Carolina Tar Heels",
    "UC San Diego Tritons",
    "Yale Bulldogs",
    "Lipscomb Bisons",
    "Bryant Bulldogs",
    "Alabama State Hornets",
];

pub const west_team_names: [&'static str; 16] = [
    "Florida Gators",
    "St. John's Red Storm",
    "Texas Tech Red Raiders",
    "Maryland Terrapins",
    "Memphis Tigers",
    "Missouri Tigers",
    "Kansas Jayhawks",
    "UConn Huskies",
    "Oklahoma Sooners",
    "Arkansas Razorbacks",
    "Drake Bulldogs",
    "Colorado State Rams",
    "Grand Canyon Lopes",
    "UNC Wilmington Seahawks",
    "Omaha Mavericks",
    "Norfolk State Spartans",
];

pub const east_team_names: [&'static str; 16] = [
    "Duke Blue Devils",
    "Alabama Crimson Tide",
    "Wisconsin Badgers",
    "Arizona Wildcats",
    "Oregon Ducks",
    "BYU Cougars",
    "Saint Mary's Gaels",
    "Mississippi State Bulldogs",
    "Baylor Bears",
    "Vanderbilt Commodores",
    "VCU Rams",
    "Liberty Flames",
    "Akron Zips",
    "Montana Grizzlies",
    "Robert Morris Colonials",
    "Mount St. Mary's Mountaineers",
];

pub const midwest_team_names: [&'static str; 16] = [
    "Houston Cougars",
    "Tennessee Volunteers",
    "Kentucky Wildcats",
    "Purdue Boilermakers",
    "Clemson Tigers",
    "Illinois Fighting Illini",
    "UCLA Bruins",
    "Gonzaga Bulldogs",
    "Georgia Bulldogs",
    "Utah State Aggies",
    "Xavier Musketeers",
    "McNeese Cowboys",
    "High Point Panthers",
    "Troy Trojans",
    "Wofford Terriers",
    "SIU Edwardsville Cougars",
];

pub fn create_region_name_to_vector_of_teams_map() -> HashMap<String, Vec<Team>> {
    let strength_map = read_csv_to_hashmap("strength.csv").unwrap();

    let mut region_team_names_map: HashMap<String, [&'static str; 16]> = HashMap::new();
    region_team_names_map.insert("south".to_string(), south_team_names);
    region_team_names_map.insert("west".to_string(), west_team_names);
    region_team_names_map.insert("east".to_string(), east_team_names);
    region_team_names_map.insert("midwest".to_string(), midwest_team_names);

    let region_names = vec!["south", "west", "east", "midwest"];

    let mut region_map: HashMap<String, Vec<Team>> = HashMap::new();

    for region_name in region_names.iter() {
        let team_names = region_team_names_map.get(&region_name.to_string()).unwrap();

        let mut region_teams = vec![];
        for (index, name) in team_names.iter().enumerate() {
            let seed = index as u32 + 1;
            let strength = *strength_map.get(*name).unwrap();
            let team = Team::new(name.to_string(), seed, strength);

            region_teams.push(team);
        }

        region_map.insert(region_name.to_string(), region_teams);
    }

    region_map
}

pub fn create_region_name_to_region_map<'a>(
    region_name_to_vector_of_teams_map: &HashMap<String, Vec<Team>>,
) -> HashMap<String, Region> {
    let mut region_name_to_region_map: HashMap<String, Region> = HashMap::new();

    for (region_name, vector_of_teams) in region_name_to_vector_of_teams_map {
        let region = Region::new(
            region_name.to_string(),
            &vector_of_teams[0],
            &vector_of_teams[1],
            &vector_of_teams[2],
            &vector_of_teams[3],
            &vector_of_teams[4],
            &vector_of_teams[5],
            &vector_of_teams[6],
            &vector_of_teams[7],
            &vector_of_teams[8],
            &vector_of_teams[9],
            &vector_of_teams[10],
            &vector_of_teams[11],
            &vector_of_teams[12],
            &vector_of_teams[13],
            &vector_of_teams[14],
            &vector_of_teams[15],
        );

        region_name_to_region_map.insert(region_name.to_string(), region);
    }

    region_name_to_region_map
}
