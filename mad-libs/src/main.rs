mod files;

fn main() {
    println!("Hello, world!");
    let story = files::read_file();
    let words: Vec<String> = files::parse_words(&story);
    println!("Words: {:?}", words);
    let new_story = files::replace_words(&story, words);
    println!("New Story: {}", new_story);
}
