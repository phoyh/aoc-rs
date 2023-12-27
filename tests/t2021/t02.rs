use aoc_rs::api;

trait CommandExec {
	fn down(&mut self, param: i32);
	fn forward(&mut self, param: i32);
	fn up(&mut self, param: i32);
}

struct WithAim {
	aim: i32,
	depth: i32,
	horizontal: i32,
}

impl CommandExec for WithAim {
	fn down(&mut self, param: i32) {
		self.aim += param;
	}
	fn forward(&mut self, param: i32) {
		self.depth += self.aim * param;
		self.horizontal += param;
	}
	fn up(&mut self, param: i32) {
		self.aim -= param;
	}
}

struct WoAim {
	depth: i32,
	horizontal: i32,
}

impl CommandExec for WoAim {
	fn down(&mut self, param: i32) {
		self.depth += param;
	}
	fn forward(&mut self, param: i32) {
		self.horizontal += param;
	}
	fn up(&mut self, param: i32) {
		self.depth -= param;
	}
}

fn exec(ce: &mut dyn CommandExec) {
	for line in api::parse::to_lines("2021/02.txt") {
		let split_line = line.split(" ").collect::<Vec<&str>>();
		let param = split_line[1].parse::<i32>().unwrap();
		match split_line[0] {
			"forward" => ce.forward(param),
			"down" => ce.down(param),
			"up" => ce.up(param),
			s => panic!("What is {}", s)
		}
	}
}

#[test]
fn part1() {
	let mut com_exec = WoAim { depth: 0, horizontal: 0 };
	exec(&mut com_exec);
	assert_eq!(1882980, com_exec.horizontal * com_exec.depth);
}

#[test]
fn part2() {
	let mut com_exec = WithAim { aim: 0, depth: 0, horizontal: 0 };
	exec(&mut com_exec);
	assert_eq!(1971232560, com_exec.horizontal * com_exec.depth);
}
