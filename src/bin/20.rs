use std::collections::{HashMap, VecDeque};
use std::ops::Not;
use std::rc::Rc;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Copy, Clone)]
enum State {
    On,
    Off,
}

impl Not for State {
    type Output = State;

    fn not(self) -> Self::Output {
        match self {
            State::On => State::Off,
            State::Off => State::On,
        }
    }
}

#[derive(Debug, Clone)]
enum ModuleState {
    Flipflop { state: State },
    Conjunction { memory: HashMap<Rc<str>, Pulse> },
    Broadcast,
}

#[derive(Debug, Clone)]
struct Module {
    label: Rc<str>,
    state: ModuleState,
    destinations: Vec<Rc<str>>,
}

impl Module {
    fn new(input: &str) -> Module {
        let (module, outputs) = input.split_once(" -> ").expect("Arrow delimited");
        let outputs = outputs.split(", ").map(|entry| entry.into()).collect();

        let label = if module == "broadcaster" {
            module
        } else {
            module.get(1..).unwrap()
        }
        .into();

        let state_type: ModuleState = match module.as_bytes()[0] {
            b'&' => ModuleState::Conjunction {
                memory: HashMap::new(),
            },
            b'%' => ModuleState::Flipflop { state: State::Off },
            b'b' => ModuleState::Broadcast,
            _ => panic!("Invalid entry"),
        };

        Module {
            label,
            state: state_type,
            destinations: outputs,
        }
    }

    fn update(&mut self, signal: &Signal) -> Option<Pulse> {
        match &mut self.state {
            ModuleState::Flipflop { state } => {
                return if signal.pulse == Pulse::Low {
                    *state = !*state;
                    Some(match state {
                        State::On => Pulse::High,
                        State::Off => Pulse::Low,
                    })
                } else {
                    None
                }
            }
            ModuleState::Conjunction { memory } => {
                memory.insert(signal.from.clone(), signal.pulse);
                return if memory.iter().all(|(_, &pulse)| pulse == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                };
            }
            ModuleState::Broadcast => Some(signal.pulse),
        }
    }
}

#[derive(Debug)]
struct Signal {
    from: Rc<str>,
    to: Rc<str>,
    pulse: Pulse,
}

// Euclid was a genius so let's use that
fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

// get least common multiple by relationship of greatest common denominator
fn lcm(a: u64, b: u64) -> u64 {
    return (a * b) / gcd(a, b);
}

fn construct_modules(input: &str) -> HashMap<Rc<str>, Module> {
    let mut modules: HashMap<Rc<str>, Module> = input
        .trim_end()
        .lines()
        .map(|line| {
            let module = Module::new(line);
            (module.label.clone(), module)
        })
        .collect();

    // set all of the connections for a conjunction to the initial Low state
    for (sender_label, module) in modules.clone().iter() {
        for receiver_label in &module.destinations {
            if let Some(receiver) = modules.get_mut(receiver_label) {
                if let ModuleState::Conjunction { ref mut memory } = receiver.state {
                    memory.insert(sender_label.clone(), Pulse::Low);
                }
            }
        }
    }

    return modules;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut modules: HashMap<Rc<str>, Module> = construct_modules(input);

    let mut total_lows = 0;
    let mut total_highs = 0;
    let mut queue = VecDeque::new();

    for _ in 0..1000 {
        queue.push_back(Signal {
            from: "button".into(),
            to: "broadcaster".into(),
            pulse: Pulse::Low,
        });

        while let Some(signal) = queue.pop_front() {
            if signal.pulse == Pulse::Low {
                total_lows += 1;
            } else {
                total_highs += 1;
            }

            if let Some(module) = modules.get_mut(&signal.to) {
                if let Some(pulse) = module.update(&signal) {
                    for destination in &module.destinations {
                        queue.push_back(Signal {
                            from: module.label.clone(),
                            to: destination.clone(),
                            pulse,
                        })
                    }
                }
            }
        }
    }

    Some(total_lows * total_highs)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut modules: HashMap<Rc<str>, Module> = construct_modules(input);

    let mut queue = VecDeque::new();

    let penultimate_module = modules
        .iter()
        .find(|(_, module)| module.destinations.contains(&"rx".into()))
        .expect("Input crafted to have an 'rx' module destination.")
        .1;

    let antepenultimate_modules: Vec<_> = modules
        .iter()
        .filter(|(_, module)| module.destinations.contains(&penultimate_module.label))
        .map(|entry| entry.0.clone())
        .collect();

    let mut count = antepenultimate_modules.len();

    let mut accumulator = 1;
    'outer: for press in 1.. {
        queue.push_back(Signal {
            from: "button".into(),
            to: "broadcaster".into(),
            pulse: Pulse::Low,
        });

        while let Some(signal) = queue.pop_front() {
            if antepenultimate_modules.contains(&signal.from) && signal.pulse == Pulse::High {
                accumulator = lcm(accumulator, press);
                count -= 1;
                if count == 0 {
                    break 'outer;
                }
            }
            if let Some(module) = modules.get_mut(&signal.to) {
                if let Some(pulse) = module.update(&signal) {
                    for destination in &module.destinations {
                        queue.push_back(Signal {
                            from: module.label.clone(),
                            to: destination.clone(),
                            pulse,
                        })
                    }
                }
            }
        }
    }

    Some(accumulator)
}

advent_of_code::main!(20);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 20));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 20));
        assert_eq!(result, None);
    }
}
