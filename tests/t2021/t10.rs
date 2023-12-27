use aoc_rs::api::parse;
use itertools::Itertools;

fn get_stack_and_error(line: &str) -> (Vec<char>,Option<char>) {
	let mut stack = Vec::new();
	for c in line.chars() {
		match c {
			'(' => stack.push(')'),
			'[' => stack.push(']'),
			'{' => stack.push('}'),
			'<' => stack.push('>'),
			_ => {
				if Some(&c) != stack.last() {
					return (stack, Some(c))
				}
				stack.pop();
			}
		}
	}
	(stack, None)
}

fn get_error_score(line: &str) -> i32 {
	let (_, error) = get_stack_and_error(line);
	if let Some(error_char) = error {
		match error_char {
			')' => return 3,
			']' => return 57,
			'}' => return 1197,
			_ => return 25137,
		}
	}
	0
}

fn get_completion_score(line: &str) -> i64 {
	let (mut stack, _) = get_stack_and_error(line);
	stack.reverse();
	stack.iter()
		.map(|&c| 1 + ")]}>".chars().position(|cc| c == cc).unwrap() as i64)
		.fold(0, |s,score| 5 * s + score)
}

#[test]
fn part1() {
	let error_score = parse::to_lines("2021/10.txt").iter()
		.map(|s| get_error_score(s))
		.filter(|&score| score > 0)
		.sum();
	assert_eq!(288291, error_score);
}

#[test]
fn part2() {
	let mut completion_scores = parse::to_lines("2021/10.txt").iter()
		.filter(|s| get_error_score(s) == 0)
		.map(|s| get_completion_score(s))
		.collect_vec();
	completion_scores.sort();
	assert_eq!(820045242, completion_scores[(completion_scores.len() - 1) / 2]);
}
