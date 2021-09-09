use rand::Rng;
use std::fs;
use regex::Regex;

pub fn read_file() -> String {
    // Pick a random number to choose the file
    // Read that file and return string
    let mut generator = rand::thread_rng();
    let story_number = generator.gen_range(1..21);
    println!("Story number: {}", story_number);
    //let filename = format!("./Stories/story_{}.txt", story_number);
    let filename = "./Stories/story_1.txt";
    format!("Hello, {}!", "world");
    let story = fs::read_to_string(filename)
        .expect("File not found or could not be read");
    return story;
}

pub fn parse_words(story:&str) -> Vec<String> {
    // Read tokens ({}) into vector of strings
    // return vector

    let mut word_vec: Vec<String> = vec![];
    let regex = Regex::new(r"\{[ \w\d_()]*\}").unwrap();

    for caps in regex.captures_iter(story) {
        let mut reg_match = caps.get(0).unwrap().as_str().to_string();
        reg_match = reg_match.replace("{ ", "");
        reg_match = reg_match.replace(" }", "");
        word_vec.push(reg_match);
    }

    return word_vec;
}

pub fn replace_words(story:&str, words:Vec<String>) -> String {
    let regex = Regex::new(r"\{[ \w\d_()]*\}").unwrap();
    let mut result: String = story.to_string();
    for word in words.iter() {
        result = regex.replace(&result, word.to_uppercase()).to_string();
    }

    return result;
}