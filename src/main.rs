use std::fs;
use std::io;
use std::collections::HashMap;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};

// Global, constant list of separators.
// TODO: Let the user supply its own file with separators.
// TODO: Since I don't wanna deal with having repeated keys in the hashmap for capitalized or non capitalized letters, just fugging turn them into non capitalized ones when checking to replace them. 5000 IQ, I know. I know.
lazy_static! {
	static ref SEPARATORS: &'static str = "-#¬_~=*+─ ";
	static ref SEPARATORS_LENGTH: usize = SEPARATORS.chars().count();
	static ref TYPO_CHARACTERS: HashMap<&'static str, &'static str> = HashMap::from([
		("a", "4"),
		("e", "3"),
		("i", "1"),
		("o", "0"),
		("l", "!"),
		("s", "5")										
	]);
}

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
	for (_index, &item) in string_as_bytes.iter().enumerate() {
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

// Modify some of the characters of a given word, aka "typo-ify" them.
fn typoify_word(input_word: &str) -> String {
	let mut rng = thread_rng();
	// 50% chance to typoify a word.
	let typoify_chance = rng.gen_range(0..=1);
	if typoify_chance != 1 {
		return input_word.to_string();
	}
	let mut typoified_word = Default::default();
	for character in input_word.chars() {
		// operation where each letter is matched to a letter in the hash map and changed accordingly if present
		match TYPO_CHARACTERS.get(&*character.to_string()) {
			Some(typo_character) => typoified_word += &*typo_character.to_string(),
			None => typoified_word += &*character.to_string()
		}
	}
	return typoified_word;
}

// Constucts the passphrase.
// Using borrows for the input args because I plan on letting this data be used to show the parameters used at the end.
fn construct_passphrase(dictionary_contents: &String, wordcount: &usize, passphrase_length: &usize, typoify: &u8) -> String {
	let mut length = *passphrase_length;
	let mut passphrase: String = Default::default();
	// add up the words and their separators until the requested length is 0
	while length > 0 {
		let mut rng = thread_rng();
		let requested_word_linenumber = rng.gen_range(1..=*wordcount);
		// Re-seeding... is this even necessary? I don't know
		rng = thread_rng();
		let requested_character_index = rng.gen_range(1..=*SEPARATORS_LENGTH);
		// current word we got
		let current_word = get_word(dictionary_contents, requested_word_linenumber);
		if *typoify == 1 {
			passphrase += &typoify_word(current_word);
		}
		else {
			 passphrase += current_word;
		}
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
	println!("Do you want to typoify some words of the passphrase? Currently, there's a 50% chance for a word in the passphrase to be typoified.");
	let mut requested_typoify = String::new();
	io::stdin().read_line(&mut requested_typoify).expect("Somehow failed to read the input.");
	println!("Passphrase: {}", construct_passphrase(&dictionary_contents, &dictionary_wordcount, &requested_passphrase_length.trim().parse::<usize>().unwrap(), &requested_typoify.trim().parse::<u8>().unwrap()));
}
