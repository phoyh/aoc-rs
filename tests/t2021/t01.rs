use aoc_rs::api;

fn get_depth_list() -> Vec<i32> {
	api::parse::to_lines("2021/01.txt")
		.iter().filter_map(|s| s.parse().ok()).collect()

}

#[test]
fn part1() {
	let depth_list = get_depth_list();
	let increases = depth_list.iter().enumerate()
		.filter(|(i,e)| *i > 0 && *e > depth_list.get(i - 1).unwrap())
		.count();
	assert_eq!(1482, increases);
}

#[test]
fn part2() {
	let depth_list = get_depth_list();
	let increases = depth_list.iter().enumerate()
		.filter(|(i,e)| *i > 2 && *e > depth_list.get(i - 3).unwrap())
		.count();
	assert_eq!(1518, increases);
}
