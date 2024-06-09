use reqwest;
use std::io;
use std::io::Write;

#[tokio::main]
async fn main() {
    println!("Hello, welcome to Hangman!");

    loop {
        let difficulty = select_difficulty();
        let max_attempts = match difficulty.as_str() {
            "easy" => 10,
            "medium" => 7,
            "hard" => 4,
            _ => 7, // default to medium if somehow an invalid input sneaks through
        };

        let mut hangman_stage = 0; // Initialize the hangman stage

        let secret_word = match get_random_word().await {
            Ok(word) => word,
            Err(e) => {
                println!("Failed to get a random word: {}", e);
                return;
            }
        };

        let mut guessed_word: Vec<char> = vec!['_'; secret_word.len()];
        let mut current_attempts = max_attempts;
        let mut guessed_letters = String::new();

        while current_attempts > 0 {
            print_game_status(&guessed_word, current_attempts, &guessed_letters, hangman_stage, max_attempts);

            let guess = get_guess();
            if guessed_letters.contains(guess) {
                println!("You already guessed that letter!");
                continue;
            }

            guessed_letters.push(guess);

            if secret_word.contains(guess) {
                update_guessed_word(&secret_word, &mut guessed_word, guess);
                if !guessed_word.contains(&'_') {
                    println!("Congratulations, you won! The word was: {}", secret_word);
                    break;
                }
            } else {
                current_attempts -= 1;
                hangman_stage += 1; // Increment the hangman stage
                println!("Incorrect! You have {} attempts left.", current_attempts);
            }

            if current_attempts == 0 {
                println!("Game over! The word was: {}", secret_word);
            }
        }

        if !play_again() {
            break;
        }
    }
}

async fn get_random_word() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://random-word-api.herokuapp.com/word?number=1")
        .await?
        .json::<Vec<String>>()
        .await?;
    Ok(response[0].clone())
}

fn print_game_status(
    guessed_word: &[char],
    attempts: usize,
    guessed_letters: &str,
    hangman_stage: usize,
    max_attempts: usize,
) {
    println!("\nWord: {}", guessed_word.iter().collect::<String>());
    println!("Attempts remaining: {}", attempts);
    println!("Guessed letters: {}", guessed_letters);
    println!("Hangman:");
    print_hangman(hangman_stage, max_attempts);
}

fn get_guess() -> char {
    print!("Enter your guess: ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    guess.trim().chars().next().unwrap_or('_')
}

fn update_guessed_word(secret_word: &str, guessed_word: &mut Vec<char>, guess: char) {
    for (i, c) in secret_word.chars().enumerate() {
        if c == guess {
            guessed_word[i] = guess;
        }
    }
}

fn play_again() -> bool {
    print!("Do you want to play again? (y/n): ");
    io::stdout().flush().unwrap();

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read line");
    match answer.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        _ => false,
    }
}

fn select_difficulty() -> String {
    println!("Select difficulty level: easy, medium, hard");
    let mut difficulty = String::new();
    io::stdin().read_line(&mut difficulty).expect("Failed to read line");
    let difficulty = difficulty.trim().to_lowercase();
    match difficulty.as_str() {
        "easy" | "medium" | "hard" => difficulty,
        _ => {
            println!("Invalid difficulty level. Defaulting to medium.");
            String::from("medium")
        }
    }
}

fn print_hangman(stage: usize, max_attempts: usize) {
    let stages = match max_attempts {
        10 => vec![
            "  _______\n |       |\n |\n |\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |       |\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |      /|\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |      /|\\\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |      /|\\\n |      /\n |\n_|___",
            "  _______\n |       |\n |       O\n |      /|\\\n |      / \\\n |\n_|___",
            "  _______\n |       |\n |       O\n |      /|\\\n |      / \\\n |     /\n_|___",
            "  _______\n |       |\n |       O\n |      /|\\\n |      / \\\n |     / \\\n_|___",
            "  _______\n |       |\n |       O\n |      /|\\\n |      / \\\n |     / \\\n_|___\n\nYou are dead!",
        ],
        7 => vec![
            "  _______\n |       |\n |\n |\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |       |\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |      /|\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |      /|\\\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |      /|\\\n |      / \n |\n_|___",
            "  _______\n |       |\n |       O\n |      /|\\\n |      / \\\n |\n_|___\n\nYou are dead!",
        ],
        4 => vec![
            "  _______\n |       |\n |\n |\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |       |\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |      /|\\\n |      / \\\n |\n_|___\n\nYou are dead!",
        ],
        _ => vec![
            "  _______\n |       |\n |\n |\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |       |\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |      /|\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |      /|\\\n |\n |\n |\n_|___",
            "  _______\n |       |\n |       O\n |      /|\\\n |      / \n |\n_|___",
            "  _______\n |       |\n |       O\n |      /|\\\n |      / \\\n |\n_|___\n\nYou are dead!",
        ],
    };

    println!("{}", stages[stage.min(stages.len() - 1)]);
}
