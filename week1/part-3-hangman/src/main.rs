// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn print_status(guessing_word: &Vec<char>, guessed_letters: &String, remain_guesses: &u32) {
    println!("The word so far is {}", guessing_word.iter().collect::<String>());
    println!("You have guessed the following letters: {}", guessed_letters);
    println!("You have {} guesses left", remain_guesses);
}

fn read_guess_letter() -> char {
    print!("Please guess a letter: ");
    io::stdout().flush().expect("Error flushing stdout.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Error reading line.");
    guess.chars().next().unwrap()
}

fn get_correct_indexs(secret_word: &String, guess_letter: &char) -> Vec<usize> {
    secret_word
        .char_indices()
        .filter(|(_, char)|char == guess_letter)
        .map(|(idx, _)|idx)
        .collect()
}

fn check_all_correct(guessing_word: &Vec<char>, secret_word: &String) -> bool {
    let guessing_word_str = guessing_word.iter().collect::<String>();
    guessing_word_str == *secret_word
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)
    println!("Welcome to CS110L Hangman!");
    let mut remain_guesses = NUM_INCORRECT_GUESSES;
    let mut guessing_word = vec!['-'; secret_word.len()];
    let mut guessed_letters = String::new();
    while remain_guesses > 0 || check_all_correct(&guessing_word, &secret_word) {
        print_status(&guessing_word, &guessed_letters, &remain_guesses);
        let guess_letter = read_guess_letter();
        guessed_letters.push(guess_letter);
        let correct_indexs = get_correct_indexs(&secret_word, &guess_letter);
        if correct_indexs.len() > 0 {
            correct_indexs.iter().for_each(|idx| {
                guessing_word[*idx] = guess_letter;
            });
        } else {
            remain_guesses -= 1;
            println!("Sorry, that letter is not in the word")
        }
        println!();
    }

    if remain_guesses <= 0 {
        println!("Sorry, you ran out of guesses!");
    } else {
        println!("Congratulations you guessed the secret word: {}!", secret_word);
    }
}
