use aoc_rs::api::parse;

fn bin_to_int(s: &String) -> u64 {
	u64::from_str_radix(s.as_str(), 2).unwrap()
}

fn get_lines() -> Vec<String> {
	parse::to_lines("2021/03.txt")
}

fn get_one_occurrance(lines: &Vec<String>, pos: usize) -> usize {
	lines.iter().filter(|s| s.chars().nth(pos) == Some('1')).count()
}

type GetWinner = dyn Fn(usize, usize) -> char;

fn reduce_by_bits(orig_lines: &Vec<String>, w: &GetWinner) -> String {
	let mut lines = orig_lines.clone();
	let mut pos = 0;
	while lines.len() > 1 {
		let o_oc = get_one_occurrance(&lines, pos);
		let winner_bit = w(o_oc, lines.len());
		lines.retain(|s| s.chars().nth(pos) == Some(winner_bit));
		pos += 1;
	}
	lines[0].clone()
}

#[test]
fn part1() {
	let lines = get_lines();
	let bit_len = lines[0].len();
	let mut gamma = "".to_string();
	let mut epsilon = "".to_string();
	for i in 0..bit_len {
		if get_one_occurrance(&lines, i) * 2 > lines.len() {
			gamma += "1";
			epsilon += "0";
		} else {
			gamma += "0";
			epsilon += "1";
		};
	}
	assert_eq!(2648450, bin_to_int(&gamma) * bin_to_int(&epsilon));
}

#[test]
fn part2() {
	let lines = get_lines();
	let o2 = reduce_by_bits(&lines,
		&|o_oc, len| if o_oc * 2 >= len { '1' } else { '0' });
	let co2 = reduce_by_bits(&lines,
		&|o_oc, len| if o_oc * 2 < len { '1' } else { '0' });
	assert_eq!(2845944, bin_to_int(&o2) * bin_to_int(&co2));
}