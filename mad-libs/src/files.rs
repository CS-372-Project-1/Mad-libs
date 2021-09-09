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
    let words = story.split("{ ").flat_map(|s| s.split(" }"));
    let split_vec = words.collect::<Vec<&str>>();
    let mut word_vec: Vec<String> = vec![];
    for word in split_vec.iter() {
        if !word.contains(" ") {
            word_vec.push(word.to_string());
        }
    }

    return word_vec;
}

pub fn replace_words(story:&str, words:Vec<String>) -> String {
    let regex = Regex::new(r"/\{(?P<word>[^{]*)([^}])\}/g").unwrap();
    let mut result: String = story.to_string();
    // for word in words.iter() {
    //     //result = regex.replace(&result, word.to_uppercase()).to_string();
    //     println!("Regex: {:?}", regex.replace(&result, |caps: &Captures|));
    // }

    for caps in regex.captures_iter(story) {
        println!("Word: {:?}",
                 &caps["word"]);
    }

    return result;
}