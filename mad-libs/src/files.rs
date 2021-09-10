use rand::Rng;
use std::fs;

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
