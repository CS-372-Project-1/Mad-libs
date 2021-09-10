use std::collections::HashMap;
use std::io;

mod files;
mod game;

fn main() {
    println!("Hello, and welcome to Mad Libs!");
    println!("Let's get started!");
    let story = files::read_file();
    let words: Vec<String> = game::parse_words(&story);
    println!("Words: {:?}", words);
    // Need to get user input here
    let new_story = game::replace_words(&story, words); // should be called with user input
    println!("New Story: {}", new_story);
}

// Use hash map: loop through word types, add to map if not seen,
// otherwise add user input to 

pub fn initialize_map(words:Vec<String>) -> HashMap<String, Vec<String>> {
    let mut input_map: HashMap<String, Vec<String>> = HashMap::new();
    
    for word in words.iter() {
        if !input_map.contains_key(word) {
            let empty_vec: Vec<String> = Vec::new();
            input_map.insert(word.to_string(), empty_vec);
        }
    }

    return input_map;
}

pub fn fill_map(user_input:Vec<(String, String)>, map:HashMap<String, Vec<String>>)
    -> HashMap<String, Vec<String>> {
        for input_group in user_input {
            let v = map.get_mut(&input_group.0).unwrap();
            v.push(input_group.1);
        }

        return map;
}

pub fn get_user_input() {    
    let mut in_str = String::new();
    // loop to fill global hashmap
    io::stdin().read_line(&mut in_str).expect("Error: Please input a word");
}