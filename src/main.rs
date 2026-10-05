use random_word::Lang;
use std::io;

fn main() {
    println!("Guess the 5 length, english word!");
    println!("The next letter will be revealed after each fail.");
    println!();

    let wordle = get_word();
    let l = get_ascii(wordle.clone());
    let first = l[0];
    let second = l[1];
    let third = l[2];
    let forth = l[3];

    let mut counter = -1;
    
    loop {
        counter += 1;
        println!("---------------------------------");
        println!("Your guess: ");
        let mut guess = String::new();
        let _x = io::stdin().read_line(&mut guess);
        let guess = guess.trim();
        println!();
        println!();

        if wordle.eq_ignore_ascii_case(&guess) {
            println!("--- Correct. You win!");
            if counter == 0 {
                println!("First try! Damn, you're smart");
            }
            break;
        } else {
            println!("--- False.");
            if counter < 4 {
                println!("word: {} {} {} {} {}", if counter >= 0 {first} else {'_'}, if counter >= 1 {second} else {'_'}, if counter >= 2 {third} else {'_'}, if counter >= 3 {forth} else {'_'}, '_');
            }
            if counter >= 4 {
                println!("You lose. The correct word was: {}", &wordle);
                break;
            }
            
        }
    }
}

fn get_word() -> String {
    return random_word::get_len(5, Lang::En).expect("failed to get word").to_string();
}

fn get_ascii(wordle: String) -> Vec<char> {
    return wordle.chars().collect();
}