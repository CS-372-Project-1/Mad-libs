use std::collections::HashMap;
use std::io;

mod files;
mod game;

static mut MAP: HashMap<String, Vec<String>> = HashMap::new();

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

pub fn initialize_map(words:Vec<String>) {
    for word in words.iter() {
        if !MAP.contains_key(word) {
            let empty_vec: Vec<String> = Vec::new();
            MAP.insert(word.to_string(), empty_vec);
        }
    }
}

pub fn fill_map(user_input:Vec<(String, String)>) {
        for input_group in user_input {
            let v = MAP.get_mut(&input_group.0).unwrap();
            v.push(input_group.1);
        }
}

pub fn get_user_input() {    
    let mut in_str = String::new();
    // loop to fill global hashmap
    io::stdin().read_line(&mut in_str).expect("Error: Please input a word");
}