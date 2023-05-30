use std::{fs};


// Returns the word itself.
fn get_word(input_string: &String, linecount: usize) -> &str {
	let string_as_bytes = input_string.as_bytes();
	// This value helps with skipping lines
	let mut helper = linecount;
	let mut word_start_index = 0; 
	for (index, &item) in string_as_bytes.iter().enumerate() {
		if item == b'\n' {
			// Return for any word that isn't the last.
			if helper == 1 {
				return &input_string[word_start_index..index];	
			}
			// This helps skip to the word we intend to get, and sets the word_start_index to what it should be.
			if helper > 1 {
				helper -= 1;
				word_start_index = index + 1;
			}
		}
	}
	// For the last word.
	return &input_string[word_start_index..];
}

// Counts the words.
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

fn main() {
	let dictionary_file = "./src/english-dictionary.txt";
	
	let dictionary_contents = fs::read_to_string(dictionary_file).expect("Couldn't read file.");

	let test = get_word(&dictionary_contents, 61);
	let test_word = test;
	println!("The word is {}.", test_word);

	//println!("{}",dictionary_contents);
	let wordcount = count_words(&dictionary_contents);
	println!("The number of words is: {}", wordcount);
}