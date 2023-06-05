use std::fs;
use rand::{thread_rng, Rng};

// I have to fill this with more characters eventually, and also randomly get each character.
static SEPARATORS: &str = "-";

// Returns the word itself. Inputs are the world list, and the intended line number to get.
fn get_word(input_string: &String, linenumber: usize) -> &str {
	let string_as_bytes = input_string.as_bytes();
	// This value helps with skipping lines
	let mut counter = linenumber;
	let mut word_start_index = 0; 
	for (index, &character) in string_as_bytes.iter().enumerate() {
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
fn construct_passphrase(dictionary_contents: &String, wordcount: &usize, passphrase_length: usize) -> String {
	let mut length = passphrase_length;
	let mut passphrase: String = "".to_string();
	// add up the words and their separators until the requested length is 0
	while length > 0 {
		let mut rng = thread_rng();
		let requested_word_linenumber = rng.gen_range(1..=*wordcount);
		passphrase += get_word(dictionary_contents, requested_word_linenumber);
		// Avoid adding a separator at the end of the passphrase.
		if length > 1 {
			passphrase += SEPARATORS;
		}
		length -= 1;
	}
	return passphrase;
}

fn main() {
	let dictionary_file = "./src/english-dictionary.txt";
	
	let dictionary_contents = fs::read_to_string(dictionary_file).expect("Couldn't read file.");

	let test = get_word(&dictionary_contents, 61);
	let test_word = test;
	println!("The word is {}.", test_word);

	//println!("{}",dictionary_contents);
	let wordcount = count_words(&dictionary_contents);
	println!("The number of words is: {}", wordcount);

	println!("the passphrase is {}", construct_passphrase(&dictionary_contents, &wordcount, 14));
}
