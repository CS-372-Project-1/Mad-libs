use std::collections::HashMap;

mod files;
mod game;

fn main() {
    println!("Hello, and welcome to Mad Libs!");
    println!("Let's get started!");
    let story = files::read_file();
    let words: Vec<String> = files::parse_words(&story);
    println!("Words: {:?}", words);
    // Need to get user input here
    let new_story = files::replace_words(&story, words); // should be called with user input
    println!("New Story: {}", new_story);
}

// Use hash map: loop through word types, add to map if not seen,
// otherwise add user input to 

pub fn initializeMap(words:Vec<String>) -> HashMap<String, Vec<String>> {
    let mut input_map: HashMap<String, Vec<String>> = HashMap::new();
    
    for word in words.iter() {
        if !input_map.contains_key(word) {
            let empty_vec: Vec<String> = Vec::new();
            input_map.insert(word.to_string(), empty_vec);
        }
    }

    return input_map;
}