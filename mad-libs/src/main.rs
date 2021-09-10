use std::collections::HashMap;
use std::io;

mod files;
mod game;

fn main() {
    println!("Hello, and welcome to Mad Libs!");
    println!("Let's get started!");

    loop {
        let story = files::read_file();
        let words: Vec<String> = game::parse_words(&story);
        println!("Words: {:?}", words);
        let mut map = initialize_map(words);
        let matches = get_user_input(words);
        let new_story = game::replace_words(&story, fill_map(matches, map)); // should be called with user input
        println!("Your Story: {}", new_story);
        println!();
        println!("Would you like to play again? (Y/N)");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("No input found.");
        if !user_input.to_uppercase().eq("Y\n") {
            println!("Thanks for playing!");
            break;
        }
    }
}

pub fn initialize_map(words:Vec<String>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for word in words.iter() {
        if !map.contains_key(word) {
            let empty_vec: Vec<String> = Vec::new();
            map.insert(word.to_string(), empty_vec);
        }
    }

    return map;
}

pub fn fill_map(user_input:Vec<(String, String)>, map:&mut HashMap<String, Vec<String>>) -> &mut HashMap<String, Vec<String>> {
        for input_group in user_input {
            let v = map.get_mut(&input_group.0).unwrap();
            v.push(input_group.1);
        }

        return map;
}

pub fn get_user_input(words:Vec<String>) -> Vec<(String, String)> {    
    let mut v: Vec<(String, String)> = Vec::new();

    for word in words {
        println!("Enter a {}: ", word);
        let mut in_str = String::new();
        io::stdin().read_line(&mut in_str).expect("Error: Please input a word");
        v.push((word, in_str));
    }

    return v;

}