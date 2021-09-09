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