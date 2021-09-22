/*
    File: main.rs
    Authors: Elijah Acuna and Amy Paul
    Class: CSc 372, Fall 2021
    Purpose: This file contains the driver code for the Mad Libs
    game as well as the code to initiate the word map and get user input.

    Note: Rust uses the snake_case naming convention, which is followed
    throughout the program code.
*/

// The keyword use is used to import libraries into Rust code
use std::collections::HashMap;
use std::io;

// To import functions from other developer-written files, use the 
// mod keyword
mod files;
mod game;

/*
    main: Runs the game. The game is run by reading in the word tokens 
    from a story file and creating a map of word types i.e. Noun or Adjective.
    The program then gets user input for each word type and maps the input
    in a HashMap. It then replaces the tokens in the original story with the 
    user input.
*/
// Functions are defined with the fn keyword.
fn main() {
    println!("Hello, and welcome to Mad Libs!");
    println!("Let's get started!");

    // This is an infinite loop, meaning it loops until the code inside
    // tells it to break. 
    // It is being used to continue the game until the user decides to quit.
    loop {
        // To import a function from another file, use the following syntax:
        let story = files::read_file();

        // Rust has implicit typing, as above with the declaration of story.

        // However, sometimes it is helpful to explicitly type with the following
        // syntax:
        let words: Vec<String> = game::parse_words(&story);

        // By default, variables in Rust are constant and cannot be modified.
        // map is declared specifically as mut, meaning it can be changed.
        let mut map = initialize_map(&words);

        // Either values or references can be passed as function parameters.
        // If you want a parameter to be mutable, you must make it &mut.
        let matches = get_user_input(words);
        let new_story = game::replace_words(&story, fill_map(matches, &mut map)); // should be called with user input
        
        println!();
        println!("Your Story: {}", new_story);
        println!();
        println!("Would you like to play again? (Y/N)");

        // String in Rust is a wrapper class of the primitive str type.
        let mut user_input = String::new();
        
        // To get user input, import stdin from io:
        io::stdin().read_line(&mut user_input).expect("No input found.");
        if !user_input.to_uppercase().eq("Y\n") {
            println!("Thanks for playing!");

            // To break out of the infinite loop:
            break;
        }
    }
}

/*
    initialize_map: This function takes in a Vector of Strings (analogous to
    an ArrayList in Java) and creates a HashMap mapping each String to an 
    empty Vector of Strings.
*/
fn initialize_map(words:&Vec<String>) -> HashMap<String, Vec<String>> {
    // Just like in Java, Rust has a built-in HashMap class.
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    // To iterate through a Vector, you must get its iterator.
    for word in words.iter() {
        if !map.contains_key(word) {
            let empty_vec: Vec<String> = Vec::new();
            map.insert(word.to_string(), empty_vec);
        }
    }

    return map;
}

/*
    fill_map: Takes in a Vector of String Tuples and maps each second value
    to the HashMap key corresponding to the first tuple value.
*/
fn fill_map(user_input:Vec<(String, String)>, map:&mut HashMap<String, Vec<String>>) -> &mut HashMap<String, Vec<String>> {
    // Note above: To get a mutable parameter value, the parameter must be
    // declared as &mut

    for input_group in user_input {
        // Tuples can be indexed into with .# syntax.
        // Note: It is impossible to index into a tuple with a variable index.
        let v = map.get_mut(&input_group.0).unwrap();
        v.push(input_group.1);
    }

    return map;
}

/*
    get_user_input: Takes in a Vector of Strings and gets user input for
    each one. It also maps the input to the type string via a tuple and 
    stores in in a new Vector.
*/
fn get_user_input(words:Vec<String>) -> Vec<(String, String)> {   
    // Declaring a Vector of tuples: 
    let mut v: Vec<(String, String)> = Vec::new();

    for word in words {
        println!("Enter a {}: ", word);
        let mut in_str = String::new();
        io::stdin().read_line(&mut in_str).expect("Error: Please input a word");
        
        // User input includes '\n', so the following line trims it off:
        in_str = in_str.trim_end().to_string();
        v.push((word, in_str));
    }

    return v;

}