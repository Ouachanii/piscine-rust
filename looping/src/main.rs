use std::io;

fn main() {
    let riddle = "What comes once in a minute, twice in a moment, but never in a thousand years?";

    let answer = "m";
    let mut tries = 0;

    loop {
        println!("{}", riddle);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim().to_lowercase();
        tries += 1;

        if input == answer {
            println!("Number of trials: {}", tries);
            break;
        }
    }
}
