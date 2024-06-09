use reqwest; // Importing the reqwest library for making HTTP requests
use std::io; // Importing the standard input/output module
use std::io::Write; // Importing a specific trait from the standard input/output module

#[tokio::main] // Using the tokio runtime to run async functions
async fn main() {
    println!("Hello, welcome to Hangman!"); // Printing a welcome message

    loop {
        let difficulty = select_difficulty(); // Prompting the user to select difficulty
        let max_attempts = match difficulty.as_str() {
            "easy" => 10,
            "medium" => 7,
            "hard" => 4,
            _ => 7, // default to medium if somehow an invalid input sneaks through
        };

        let mut hangman_stage = 0; // Initialize the hangman stage

        // Getting a random word from an API
        let secret_word = match get_random_word().await {
            Ok(word) => word,
            Err(e) => {
                println!("Failed to get a random word: {}", e);
                return; // Exiting the program if getting a random word fails
            }
        };

        let mut guessed_word: Vec<char> = vec!['_'; secret_word.len()]; // Creating a vector to represent the guessed word
        let mut current_attempts = max_attempts; // Setting the current attempts to the maximum attempts
        let mut guessed_letters = String::new(); // Creating a string to store guessed letters

        // Main game loop
        while current_attempts > 0 {
            // Printing the current game status
            print_game_status(&guessed_word, current_attempts, &guessed_letters, hangman_stage, max_attempts);

            let guess = get_guess(); // Getting user input for a guess
            if guessed_letters.contains(guess) {
                println!("You already guessed that letter!");
                continue; // Skipping the rest of the loop iteration if the letter has already been guessed
            }

            guessed_letters.push(guess); // Adding the guessed letter to the guessed letters string

            // Checking if the guessed letter is in the secret word
            if secret_word.contains(guess) {
                // Updating the guessed word with the guessed letter
                update_guessed_word(&secret_word, &mut guessed_word, guess);
                if !guessed_word.contains(&'_') {
                    println!("Congratulations, you won! The word was: {}", secret_word); // Printing a win message if all letters have been guessed
                    break; // Exiting the loop since the game is won
                }
            } else {
                current_attempts -= 1; // Decrementing the number of attempts left
                hangman_stage += 1; // Incrementing the hangman stage
                println!("Incorrect! You have {} attempts left.", current_attempts); // Printing an incorrect guess message
            }

            if current_attempts == 0 {
                println!("Game over! The word was: {}", secret_word); // Printing a game over message if there are no attempts left
            }
        }

        if !play_again() {
            break; // Exiting the loop if the user chooses not to play again
        }
    }
}

async fn get_random_word() -> Result<String, reqwest::Error> {
    // Making an asynchronous HTTP request to get a random word from an API
    let response = reqwest::get("https://random-word-api.herokuapp.com/word?number=1")
        .await?
        .json::<Vec<String>>()
        .await?;
    Ok(response[0].clone()) // Returning the first word from the API response
}

// Function to print the current game status
fn print_game_status(
    guessed_word: &[char],
    attempts: usize,
    guessed_letters: &str,
    hangman_stage: usize,
    max_attempts: usize,
) {
    println!("\nWord: {}", guessed_word.iter().collect::<String>()); // Printing the guessed word
    println!("Attempts remaining: {}", attempts); // Printing the number of attempts remaining
    println!("Guessed letters: {}", guessed_letters); // Printing the guessed letters
    println!("Hangman:"); // Printing the hangman stage
    print_hangman(hangman_stage, max_attempts);
}

// Function to get user input for a guess
fn get_guess() -> char {
    print!("Enter your guess: "); // Prompting the user to enter a guess
    io::stdout().flush().unwrap(); // Ensuring the prompt is displayed immediately

    let mut guess = String::new(); // Creating a string to store the user input
    io::stdin().read_line(&mut guess).expect("Failed to read line"); // Reading user input from the standard input
    guess.trim().chars().next().unwrap_or('_') // Returning the first character of the trimmed input or '_' if input is empty
}

// Function to update the guessed word with the guessed letter
fn update_guessed_word(secret_word: &str, guessed_word: &mut Vec<char>, guess: char) {
    for (i, c) in secret_word.chars().enumerate() {
        if c == guess {
            guessed_word[i] = guess; // Replacing '_' with the guessed letter in the guessed word
        }
    }
}

// Function to prompt the user to play again
fn play_again() -> bool {
    print!("Do you want to play again? (y/n): "); // Prompting the user to play again
    io::stdout().flush().unwrap(); // Ensuring the prompt is displayed immediately

    let mut answer = String::new(); // Creating a string to store the user input
    io::stdin().read_line(&mut answer).expect("Failed to read line"); // Reading user input from the standard input
    match answer.trim().to_lowercase().as_str() {
        "y" | "yes" => true, // Returning true if the user wants to play again
        _ => false, // Returning false otherwise
    }
}

// Function to prompt the user to select a difficulty level
fn select_difficulty() -> String {
    println!("Select difficulty level: easy, medium, hard"); // Prompting the user to select difficulty
    let mut difficulty = String::new(); // Creating a string to store the user input
    io::stdin().read_line(&mut difficulty).expect("Failed to read line"); // Reading user input from the standard input
    let difficulty = difficulty.trim().to_lowercase(); // Converting the input to lowercase
    match difficulty.as_str() {
        "easy" | "medium" | "hard" => difficulty, // Returning the difficulty if it's valid
        _ => {
            println!("Invalid difficulty level. Defaulting to medium."); // Printing an error message for invalid input
            String::from("medium") // Defaulting to medium difficulty
        }
    }
}

// Function to print the hangman stage
fn print_hangman(stage: usize, max_attempts: usize) {
    let stages = match max_attempts {
        // Choosing the hangman stages based on the maximum attempts allowed
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

    println!("{}", stages[stage.min(stages.len() - 1)]); // Printing the current hangman stage
}

