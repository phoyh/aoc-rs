use std::collections::HashMap;

use aoc_rust::api::parse;
use itertools::Itertools;

fn get_permutations() -> Vec<HashMap<char,char>> {
	"abcdefg".chars().permutations(7)
		.map(|p|
			p.iter().enumerate()
				.map(|(i,&pc)| ("abcdefg".chars().nth(i).unwrap(), pc))
				.collect()
		).collect()
}

fn apply_permutation(permutation: &HashMap<char,char>, input: &str) -> String {
	input.chars()
		.filter_map(|c| permutation.get(&c))
		.map(|&c| c)
		.sorted()
		.join("")
}

fn get_digit_def() -> HashMap<String, u8> {
	["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"]
		.iter().enumerate()
		.map(|(i,d)| (d.to_string(), i as u8))
		.collect()
}

fn get_values(input_raw: Vec<String>) -> Vec<Vec<u8>> {
	let digit_def = get_digit_def();
	let permutations = get_permutations();
	input_raw.iter()
		.map(|l| l.split(" | ")
			.map(|c| c.split(" ").collect_vec())
			.collect_tuple().unwrap()
		)
		.map(|(ci,val)| (val,
			permutations.iter().find(|p| 
				ci.iter().all(|d|
					digit_def.contains_key(apply_permutation(p, d).as_str())
				)
			).unwrap()
		)).map(|(val,p)|
			val.iter()
				.filter_map(|v|
					digit_def.get(apply_permutation(p, v).as_str())
				).map(|&v| v).collect_vec()
		).collect_vec()
}

fn count_occ_1478(values: Vec<Vec<u8>>) -> usize {
	values.iter()
		.map(|d|
			d.iter().filter(|&&dd| dd == 1 || dd == 4 || dd == 7 || dd == 8).count()
		).reduce(|s,v| s + v).unwrap()
}

fn sum_values(values: Vec<Vec<u8>>) -> u64 {
	values.iter().map(|d|
		d.iter().fold(0u64, |s,&d| s * 10 + (d as u64))
	).reduce(|s,v| s + v).unwrap()
}

#[test]
fn part1_ex() {
	assert_eq!(26, count_occ_1478(get_values(parse::to_lines("2021/08_ex.txt"))));
}

#[test]
fn part1() {
	assert_eq!(330, count_occ_1478(get_values(parse::to_lines("2021/08.txt"))));
}

#[test]
fn part2_ex() {
	assert_eq!(61229, sum_values(get_values(parse::to_lines("2021/08_ex.txt"))));
}

#[test]
fn part2() {
	assert_eq!(1010472, sum_values(get_values(parse::to_lines("2021/08.txt"))));
}
