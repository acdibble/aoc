use std::{
    collections::{HashMap, VecDeque},
    ops::Neg,
    time::SystemTime,
};
use utils::math::traits::*;

const DATA: &'static str = include_str!("../data.txt");
const BROADCASTER: &'static str = "BROADCASTER";

type Id = &'static str;

#[derive(Debug, Clone, Copy)]
enum State {
    Off,
    On,
}

impl Neg for State {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Off => Self::On,
            Self::On => Self::Off,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum PulseKind {
    Low,
    High,
}

impl PulseKind {
    fn is_high(&self) -> bool {
        matches!(self, PulseKind::High)
    }
}

#[derive(Debug, Clone, Copy)]
struct Pulse {
    sender: Id,
    receiver: Id,
    kind: PulseKind,
}

#[derive(Debug)]
enum Module {
    Broadcaster {
        outputs: Vec<Id>,
    },
    FlipFlop {
        state: State,
        outputs: Vec<Id>,
    },
    Conjunction {
        inputs: HashMap<Id, PulseKind>,
        outputs: Vec<Id>,
    },
}

impl Module {
    fn outputs(&self) -> &[Id] {
        match self {
            Self::Conjunction { outputs, .. }
            | Self::FlipFlop { outputs, .. }
            | Self::Broadcaster { outputs } => outputs,
        }
    }
}

fn parse_line(line: &'static str) -> (Id, Module) {
    let mut parts = line.split(" -> ");

    let name = parts.next().unwrap();

    match &name[0..1] {
        "b" => (
            BROADCASTER,
            Module::Broadcaster {
                outputs: parts.next().unwrap().split(", ").collect(),
            },
        ),
        "%" => (
            &name[1..],
            Module::FlipFlop {
                state: State::Off,
                outputs: parts.next().unwrap().split(", ").collect(),
            },
        ),
        "&" => (
            &name[1..],
            Module::Conjunction {
                inputs: Default::default(),
                outputs: parts.next().unwrap().split(", ").collect(),
            },
        ),
        _ => unreachable!("{name}"),
    }
}

fn parse_modules() -> HashMap<&'static str, Module> {
    let mut modules: HashMap<_, _> = DATA.trim().lines().map(|line| parse_line(line)).collect();

    let mut outputs_to_inputs = Vec::new();

    for (k, v) in &modules {
        outputs_to_inputs.extend(v.outputs().iter().map(|o| (*o, *k)))
    }

    for (output, input) in outputs_to_inputs {
        match modules.get_mut(output) {
            Some(Module::Conjunction { inputs, .. }) => {
                inputs.insert(input, PulseKind::Low);
            }
            _ => {}
        }
    }

    modules
}

fn part_one() -> usize {
    let mut modules = parse_modules();

    let mut highs = 0;
    let mut lows = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::from([Pulse {
            sender: "",
            receiver: BROADCASTER,
            kind: PulseKind::Low,
        }]);

        while let Some(pulse) = queue.pop_front() {
            match pulse.kind {
                PulseKind::High => highs += 1,
                PulseKind::Low => lows += 1,
            }

            let module = match modules.get_mut(pulse.receiver) {
                Some(module) => module,
                None => continue,
            };

            match module {
                Module::Broadcaster { outputs } => {
                    for id in outputs {
                        queue.push_back(Pulse {
                            sender: BROADCASTER,
                            receiver: id,
                            kind: pulse.kind,
                        })
                    }
                }
                Module::FlipFlop { state, outputs } => match pulse.kind {
                    PulseKind::High => {}
                    PulseKind::Low => {
                        for output in outputs {
                            match state {
                                State::Off => queue.push_back(Pulse {
                                    sender: pulse.receiver,
                                    receiver: output,
                                    kind: PulseKind::High,
                                }),
                                State::On => queue.push_back(Pulse {
                                    sender: pulse.receiver,
                                    receiver: output,
                                    kind: PulseKind::Low,
                                }),
                            }
                        }

                        *state = -*state;
                    }
                },
                Module::Conjunction { inputs, outputs } => {
                    inputs.insert(pulse.sender, pulse.kind);

                    let kind = if inputs.values().all(|kind| kind.is_high()) {
                        PulseKind::Low
                    } else {
                        PulseKind::High
                    };

                    for output in outputs {
                        queue.push_back(Pulse {
                            sender: pulse.receiver,
                            receiver: output,
                            kind,
                        })
                    }
                }
            }
        }
    }

    highs * lows
}

fn part_two() -> usize {
    let mut modules = parse_modules();

    let rx_input = modules
        .iter()
        .find_map(|(k, v)| {
            if v.outputs().contains(&"rx") {
                Some(*k)
            } else {
                None
            }
        })
        .unwrap();

    println!("{rx_input}");

    let rx_dependencies: Vec<_> = modules
        .iter()
        .filter_map(|(k, v)| {
            if v.outputs().contains(&rx_input) {
                Some(*k)
            } else {
                None
            }
        })
        .collect();

    let mut cycle_lengths = HashMap::<Id, usize>::new();

    for press in 1.. {
        let mut queue = VecDeque::from([Pulse {
            sender: "",
            receiver: BROADCASTER,
            kind: PulseKind::Low,
        }]);

        while let Some(pulse) = queue.pop_front() {
            let module = match modules.get_mut(pulse.receiver) {
                Some(module) => module,
                None => continue,
            };

            match module {
                Module::Broadcaster { outputs } => {
                    for id in outputs {
                        queue.push_back(Pulse {
                            sender: BROADCASTER,
                            receiver: id,
                            kind: pulse.kind,
                        })
                    }
                }
                Module::FlipFlop { state, outputs } => match pulse.kind {
                    PulseKind::High => {}
                    PulseKind::Low => {
                        for output in outputs {
                            match state {
                                State::Off => queue.push_back(Pulse {
                                    sender: pulse.receiver,
                                    receiver: output,
                                    kind: PulseKind::High,
                                }),
                                State::On => queue.push_back(Pulse {
                                    sender: pulse.receiver,
                                    receiver: output,
                                    kind: PulseKind::Low,
                                }),
                            }
                        }

                        *state = -*state;
                    }
                },
                Module::Conjunction { inputs, outputs } => {
                    inputs.insert(pulse.sender, pulse.kind);

                    let kind = if inputs.values().all(|kind| kind.is_high()) {
                        PulseKind::Low
                    } else {
                        if !cycle_lengths.contains_key(pulse.receiver) {
                            cycle_lengths.insert(pulse.receiver, press);
                        }

                        PulseKind::High
                    };

                    for output in outputs {
                        queue.push_back(Pulse {
                            sender: pulse.receiver,
                            receiver: output,
                            kind,
                        })
                    }
                }
            }
        }

        if rx_dependencies
            .iter()
            .all(|dep| cycle_lengths.contains_key(dep))
        {
            return rx_dependencies
                .into_iter()
                .map(|dep| cycle_lengths[dep])
                .fold(1, |acc, n| n.lcm(acc));
        }
    }

    unreachable!()
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
