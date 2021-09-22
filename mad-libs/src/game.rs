/*
    File: game.rs
    Authors: Elijah Acuna and Amy Paul
    Class: CSc 372, Fall 2021
    Purpose: This file contains the code to get the words types from tokens
    and replace the user input back into the story.

    Note: Rust uses the snake_case naming convention, which is followed
    throughout the program code.
*/
use regex::Regex;
use std::collections::HashMap;
use rand::Rng;

/*
    parse_words: Takes in a Vector of Strings of the form "{ Noun }" and 
    uses Regex to shave off everything except the word type ("Noun").
*/
// To make a function discoverable by other files, use the pub keyword
pub fn parse_words(story:&str) -> Vec<String> {
    // An empty, non-complex Vector can be initially like so:
    let mut word_vec: Vec<String> = vec![];
    let regex = Regex::new(r"\{[ \w\d_()-]*\}").unwrap();

    for caps in regex.captures_iter(story) {
        let mut reg_match = caps.get(0).unwrap().as_str().to_string();
        reg_match = reg_match.replace("{ ", "");
        reg_match = reg_match.replace(" }", "");
        word_vec.push(reg_match);
    }

    return word_vec;
}


/*
    replace_words: Takes in a story string as well as the map of word types 
    to words, and replaces all the word tokens in the story ("{ Noun }")
    with the proper input.
*/
pub fn replace_words(story:&str, words:&mut HashMap<String, Vec<String>>) -> String {
    let regex = Regex::new(r"\{[ \w\d_()-]*\}").unwrap();
    let mut result: String = story.to_string();

    // We grab each individual token in order.
    for caps in regex.captures_iter(story) {
        let mut reg_match = caps.get(0).unwrap().as_str().to_string();
        reg_match = reg_match.replace("{ ", "");
        reg_match = reg_match.replace(" }", "");
        
        if words.contains_key(&reg_match) {
            // Grab the matching list of user-given words for the word type.
            let v = words.get_mut(&reg_match).unwrap();
            
            // Select a random word of that type.
            let mut generator = rand::thread_rng();
            let input = generator.gen_range(0..v.len());
            let element = v.get_mut(input).unwrap();

            // Replace the token in the story with the user-given word in all
            // caps.
            result = regex.replace(&result, element.to_uppercase()).to_string();
            v.remove(input);
        }
    }

    return result;
}