pub fn read_csv_to_hashmap(file_path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;

    let mut map = HashMap::new();

    // Iterate through the CSV records and insert them into the HashMap
    for result in rdr.records() {
        let record = result?;

        // Assume two columns in each row
        if record.len() >= 2 {
            let key = record[0].to_string();
            let value = record[1].to_string();
            map.insert(key, value);
        }
    }

    Ok(map)
}
