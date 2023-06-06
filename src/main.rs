use std::fs;
use std::io;
use rand::{thread_rng, Rng};

// Global, constant list of separators.
// TODO: Let the user supply its own file with separators.
static SEPARATORS: &str = "-#¬_~=*+·─̣";
static SEPARATORS_LENGTH: &usize = &6;

// Returns the word itself. Inputs are the world list, and the intended line number to get.
fn get_word(input_string: &String, linenumber: usize) -> &str {
	let dict_content_as_bytes = input_string.as_bytes();
	// This value helps with skipping lines
	let mut counter = linenumber;
	let mut word_start_index = 0; 
	for (index, &character) in dict_content_as_bytes.iter().enumerate() {
		if character == b'\n' {
			// Return for any word that isn't the last.
			if counter == 1 {
				return &input_string[word_start_index..index];	
			}
			// This helps skip to the word we intend to get, and sets the word_start_index to what it should be.
			if counter > 1 {
				counter -= 1;
				word_start_index = index + 1;
			}
		}
	}
	// For the last word.
	return &input_string[word_start_index..];
}

// Returns the requested separator.
fn get_separator(separator_index: &usize) -> String {
	let mut counter = *separator_index;
	for character in SEPARATORS.chars() {
		if counter == 1 {
			return character.to_string();
		}
		else {
			counter -= 1;	 
		}
	}
	return "".to_string();
}

// Counts the words. Input is the word list.
fn count_words(input_string: &String) -> usize {
	let string_as_bytes = input_string.as_bytes();
	let mut counter = 0;
	let mut last_character = 0;
	for (_i, &item) in string_as_bytes.iter().enumerate() {
		// If the current character is a newline, but the last one isn't a newline, add 1 to the counter.
		if item == b'\n' && last_character != b'\n' {
			counter += 1;
		}
		last_character = item;

	}
	// If the last character isn't empty nor it is a newline, add 1 to the counter (to start from 1).
	if last_character != 0 && last_character != b'\n' {
		counter += 1;
	}
	return counter;
}

// Constucts the passphrase.
fn construct_passphrase(dictionary_contents: &String, wordcount: &usize, passphrase_length: &usize) -> String {
	let mut length = *passphrase_length;
	let mut passphrase: String = "".to_string();
	// add up the words and their separators until the requested length is 0
	while length > 0 {
		let mut rng = thread_rng();
		let requested_word_linenumber = rng.gen_range(1..=*wordcount);
		// Re-seeding...
		rng = thread_rng();
		let requested_character_index = rng.gen_range(1..=*SEPARATORS_LENGTH);
		passphrase += get_word(dictionary_contents, requested_word_linenumber);
		// Avoid adding a separator at the end of the passphrase.
		if length > 1 {
			passphrase += &get_separator(&requested_character_index);
		}
		length -= 1;
	}
	return passphrase;
}

fn main() {
	let dictionary_file = "./src/english-dictionary.txt";
	
	let dictionary_contents = fs::read_to_string(dictionary_file).expect("Couldn't read file.");

	// Actual user facing cli app code
	println!("Welcome to simple-passphrase-manager, the simple passphrase manager that's currently just a passphrase generator. Hey, I'm barely learning Rust and this is a small cli app exactly for that.");
	println!("Currently this program reads the dictionary file located at {}", dictionary_file);
	println!("How many words long should the passphrase be?");
	// TODO: use either the crossterm or temion crates to avoid printing a new line when requesting input of the user.
	println!("Requested length: ");
	let mut requested_passphrase_length = String::new();
	io::stdin().read_line(&mut requested_passphrase_length).expect("Somehow failed to read the input.");
	let dictionary_wordcount = count_words(&dictionary_contents);
	println!("Requested passphrase length: {}", requested_passphrase_length);
	println!("Number of words in the dictionary: {}", dictionary_wordcount);
	println!("Passphrase: {}", construct_passphrase(&dictionary_contents, &dictionary_wordcount, &requested_passphrase_length.trim().parse::<usize>().unwrap()));
}
