use intcode::VM;
use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

fn part_one() -> i64 {
    let mut vm = VM::from(DATA);

    vm.write_input(1);
    vm.run();

    vm.read_output().unwrap()
}

fn part_two() -> i64 {
    let mut vm = VM::from(DATA);

    vm.write_input(2);
    vm.run();

    vm.read_output().unwrap()
}

fn time_it<F, T>(fun: F) -> T
where
    F: Fn() -> T,
{
    let start = SystemTime::now();
    let result = fun();
    println!("Time elapsed: {} µs", start.elapsed().unwrap().as_micros());
    result
}

fn main() {
    time_it(|| println!("part 1: {}", part_one()));
    time_it(|| println!("part 2: {}", part_two()));
}
