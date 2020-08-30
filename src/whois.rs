use std::fs;
use rand;

pub fn get_message(username: &str) -> Result<String, String> {
    let contents = fs::read_to_string("src/ids/lookup.txt");
    let contents = match contents {
        Ok(c) => { c }
        Err(e) => {
            return Err(format!("Error! Couldn't find index! {}", e));
        }
    };
    let filename = find_file(&contents, username);
    let filename = match filename {
        Some(f) => { f }
        None => {
            println!("{}", username);
            return Err(format!("Error! Couldn't find user's file! `{}`", username));
        }
    };
    let message = get_entry(filename);
    return Ok(message);
}


fn find_file(directory: &str, name: &str) -> Option<String> {
    for line in directory.lines() {
        let splitted: Vec<&str> = line.split(",").collect();
        if splitted[0] == name {
            return Some(splitted[1].to_string());
        }
    }


    return None;
}

fn get_entry(filename: String) -> String {
    let f = "src/ids/".to_owned() + &filename;
    let f2 = f.clone();
    let contents = fs::read_to_string(f2);
    let contents = match contents {
        Ok(c) => { c }
        Err(e) => { return format!("Error in reading file!{},{}", e,f); }
    };
    //select a random entry
    let lines: Vec<&str> = contents.split("\n").collect();

    let idx = rand::random::<usize>() % lines.len();

    return lines[idx].to_string();
}
