use std::fs::File;
use std::io::{BufReader, BufRead, Lines};

fn get_reader_lines(path_in_data: &str) -> Lines<BufReader<File>> {
	let file = File::open(format!("input/{}", path_in_data)).unwrap();
	let reader = BufReader::new(file);
	reader.lines()
}

pub fn to_lines(path_in_data: &str) -> Vec<String> {
	let mut result = Vec::new();
	for line in get_reader_lines(path_in_data) {
		result.push(line.unwrap());
	}
	result
}

pub fn to_line_groups(path_in_data: &str) -> Vec<Vec<String>> {
	let mut result = Vec::new();
	let mut group = Vec::new();
	for content in get_reader_lines(path_in_data).flatten() {
		if content.is_empty() {
			if !group.is_empty() {
				result.push(group);
				group = Vec::new();
			}
		} else {
			group.push(content);
		}
	}
	if !group.is_empty() {
		result.push(group);
	}
	result
}