use std::{fs, string};


// essentially getline from c++
// returns the word itself
fn get_word(input_string: &String, linecount: usize) -> &str {
	let string_as_bytes = input_string.as_bytes();
	// This value helps with skipping lines
	let mut helper = linecount;
	let mut word_start_index = 0; 
	for (index, &item) in string_as_bytes.iter().enumerate() {
		if item == b'\n' {
			// Return for any word that isn't the last.
			if helper == 1 {
				println!("a");
				return &input_string[word_start_index..index];	
			}
			// This helps skip to the word we intend to get, and sets the word_start_index to what it should be.
			if helper > 1 {
				println!("b");
				helper -= 1;
				word_start_index = index + 1;
			}
		}
	}
	// For the last word.
	return &input_string[word_start_index..];
}

// needs to be fixed still
fn count_words(input_string: &String) -> usize {
	let string_as_bytes = input_string.as_bytes();
	let mut counter = 0;
	for (_i, &item) in string_as_bytes.iter().enumerate() {
		if item == b'\n'  {
			counter += 1;
		}	
	}
	// When the EOF is detected (because the previous for loop ended) and counter is more than zero (because there's at least one word, add 1 to the counter)
	// Not actually detecting the EOF, but hey, I'm still learning.
	// Also, doesn't work if there's just one word without a newline.
	if counter > 0 {
		counter += 1;
	}
	return counter;
}

fn main() {
	let dictionary_file = "./src/english-dictionary.txt";
	
	let dictionary_contents = fs::read_to_string(dictionary_file).expect("Couldn't read file.");

	let test = get_word(&dictionary_contents, 1);
	let test_word = test;
	println!("The word is {}.", test_word);

	//println!("{}",dictionary_contents);
	let wordcount = count_words(&dictionary_contents);
	println!("The number of words is: {}", wordcount);
}