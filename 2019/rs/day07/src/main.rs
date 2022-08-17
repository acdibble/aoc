use intcode::VM;
use std::time::SystemTime;
use utils::permute;

const DATA: &'static str = include_str!("../data.txt");

fn part_one() -> i32 {
    let opcodes = VM::parse_intcodes(DATA);
    let mut vm = VM::from(&opcodes);
    let mut max_signal = 0;

    for phase_settings in permute([0, 1, 2, 3, 4]) {
        let mut output_signal = 0;

        for setting in phase_settings {
            vm.reset(&opcodes);
            vm.write_input(setting);
            vm.write_input(output_signal);
            vm.run();
            output_signal = vm.read_output().unwrap();
        }

        max_signal = max_signal.max(output_signal);
    }

    max_signal
}

fn part_two() -> i32 {
    let opcodes = VM::parse_intcodes(DATA);
    let mut max_signal = 0;
    let mut amps = [
        VM::from(&opcodes),
        VM::from(&opcodes),
        VM::from(&opcodes),
        VM::from(&opcodes),
        VM::from(&opcodes),
    ];

    for phase_settings in permute([5, 6, 7, 8, 9]) {
        for (amp, setting) in amps.iter_mut().zip(phase_settings.iter()) {
            amp.reset(&opcodes);
            amp.write_input(*setting);
        }

        let mut output_signal = 0;
        'feedback: loop {
            for amp in &mut amps {
                amp.write_input(output_signal);
                amp.run();
                match amp.read_output() {
                    Some(value) => output_signal = value,
                    _ => break 'feedback,
                }
            }
        }

        max_signal = max_signal.max(output_signal);
    }

    max_signal
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
