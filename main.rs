const GAMES: &str include_str!("./answers.txt")


fn main() {
    let guesser = Naive::new();
    for answer in GAMES.split_whitespace() {
        play(answer, &mut guesser)
    }
    println("Hello, world!");
}

 
