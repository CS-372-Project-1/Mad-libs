use rand::Rng;
use std::fs;
use regex::Regex;
use std::collections::HashMap;

pub fn read_file() -> String {
    // Pick a random number to choose the file
    // Read that file and return string
    let mut generator = rand::thread_rng();
    let story_number = generator.gen_range(1..21);
    println!("Story number: {}", story_number);
    let filename = format!("./Stories/story_{}.txt", story_number);
    let story = fs::read_to_string(filename)
        .expect("File not found or could not be read");
    return story;
}

pub fn parse_words(story:&str) -> Vec<String> {
    // Read tokens ({}) into vector of strings
    // return vector

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

pub fn replace_words(story:&str, words:HashMap<String, Vec<String>>) -> String {
    let regex = Regex::new(r"\{[ \w\d_()-]*\}").unwrap();
    let mut result: String = story.to_string();
    // for word in words.iter() {
    //     result = regex.replace(&result, word.to_uppercase()).to_string();
    // }

    for caps in regex.captures_iter(story) {
        let mut reg_match = caps.get(0).unwrap().as_str().to_string();
        reg_match = reg_match.replace("{ ", "");
        reg_match = reg_match.replace(" }", "");
        if words.contains_key(&reg_match) {
            let mut v = words.get_mut(&reg_match).unwrap();
            let mut generator = rand::thread_rng();
            let input = generator.gen_range(0..v.len());
            let element = v.get(input).unwrap();
            v.remove(input);
            result = regex.replace(story, element.to_uppercase()).to_string();
        }
    }

    return result;
}