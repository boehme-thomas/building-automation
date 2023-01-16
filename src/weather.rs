use std::fs;

/// Loads weather data via a file and extracts only the data between `start_date` and `end_date` and in the given column/s.
/// <br/> Note that `start_date` and `end_date` have to be of the date pattern in the specific data set.
/// <br/> Also note that the data filed must be given in the column vector, or it will be deleted.
pub fn load_weather_data_dwd(path:String, start_date:&str, end_date: &str, column:Vec<u32>, separation_character: &str) -> Result<Vec<String>, std::io::Error> {
    let data = fs::read_to_string(path)?;
    let new_data: Vec<&str> = data.split("\n").collect();
    let mut string_data = Vec::<String>::new();
    let mut add = false;
    for i in 0..new_data.len() {
        if new_data[i].contains(start_date) {
            add = true;
        }
        if add {
            let split_data : Vec<&str> = new_data[i].split(separation_character).collect();
            /*for l in 0..split_data.len() {
                split_data[l] = "";
            }*/
            let mut new_created_data = String::new();
            for j in &column {
                new_created_data = new_created_data + split_data[*j as usize] + separation_character;
            }
            string_data.push(new_created_data.to_string());
        }
        if new_data[i].contains(end_date) {
            add = false;
        }
    }
    Ok(string_data)
}