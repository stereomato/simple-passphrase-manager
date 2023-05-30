use std::{fs, string};


// essentially getline from c++
// returns the word itself, and the index of where the cursor was at the time the word was returned
fn get_word(input_string: &String, mut last_index: usize) -> (&str, usize) {
	let string_as_bytes = input_string.as_bytes();

	for (i, &item) in string_as_bytes.iter().enumerate() {
		last_index = i;
		if item == b'\n' {
			return (&input_string[last_index..i], last_index);
		}
	}
	return (&input_string[..], last_index);
}

fn count_words(input_string: &String) -> usize {
	let string_as_bytes = input_string.as_bytes();
	let mut counter = 0;
	for (i, &item) in string_as_bytes.iter().enumerate() {
		last_index = i;
		if item == b'\n' {
			
		}
	}
	return (&input_string[..], last_index);
}

fn main() {
	let dictionary_file = "./src/english-dictionary.txt";
	
	let dictionary_contents = fs::read_to_string(dictionary_file).expect("Couldn't read file.");

	let test = get_word(&dictionary_contents, 0);
	let test_word = test.0;
	let test_index = test.1;
	println!("The first word is {}, and the last_index is {}", test_word, test_index);

	println!("{}",dictionary_contents);

}