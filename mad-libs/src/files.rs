/*
    File: files.rs
    Authors: Elijah Acuna and Amy Paul
    Class: CSc 372, Fall 2021
    Purpose: This file contains the code to read a random story file.

    Note: Rust uses the snake_case naming convention, which is followed
    throughout the program code.
*/

use rand::Rng;
use std::fs;

/*
    read_file: Reads in a String from a random story file and returns it.
*/
pub fn read_file() -> String {
    let mut generator = rand::thread_rng();
    // There are 20 stories, we pick a random one.
    let story_number = generator.gen_range(1..21);
    println!("Story number: {}", story_number);

    // format! allows you to format a string variable as shown below:
    let filename = format!("./Stories/story_{}.txt", story_number);
    
    // Read in story from the chosen file.
    let story = fs::read_to_string(filename)
        .expect("File not found or could not be read");
    return story;
}
