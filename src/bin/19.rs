use crate::Workflow::{Accept, Reject, GOTO, GREATER, LESS};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Workflow<'a> {
    Accept,
    Reject,
    GOTO(&'a str),                  // (label)
    GREATER(&'a str, u64, &'a str), // (rating label, comparison value, label)
    LESS(&'a str, u64, &'a str),    // (rating label, comparison value, label)
}

fn parse(input: &str) -> (HashMap<&str, Vec<Workflow>>, Vec<HashMap<&str, u64>>) {
    let mut workflows = HashMap::new();
    let mut gears = Vec::new();

    let (workflow_string, gear_string) = input
        .split_once("\n\n")
        .expect("Values split by empty line.");

    // build workflow
    workflow_string.lines().for_each(|line| {
        let mut rules = Vec::new();
        let (label, rest) = line.split_once("{").unwrap();
        rest.trim_end_matches("}")
            .split(",")
            .into_iter()
            .for_each(|rule| {
                if rule == "A" {
                    rules.push(Accept);
                } else if rule == "R" {
                    rules.push(Reject);
                } else if rule.contains(">") {
                    let (comparison, label) = rule.split_once(":").unwrap();
                    let (grade, value) = comparison.split_once(">").unwrap();
                    rules.push(GREATER(grade, value.parse().unwrap(), label));
                } else if rule.contains("<") {
                    let (comparison, label) = rule.split_once(":").unwrap();
                    let (grade, value) = comparison.split_once("<").unwrap();
                    rules.push(LESS(grade, value.parse().unwrap(), label));
                } else {
                    rules.push(GOTO(rule));
                }
            });

        workflows.insert(label, rules);
    });

    // populate gears
    gear_string.lines().for_each(|line| {
        let mut ratings = HashMap::new();
        line.trim_start_matches("{")
            .trim_end_matches("}")
            .split(",")
            .into_iter()
            .for_each(|rating| {
                let (category, value) = rating.split_once("=").unwrap();
                ratings.insert(category, value.parse().unwrap());
            });
        gears.push(ratings);
    });

    (workflows, gears)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (workflow, gears) = parse(input.trim_end());

    let mut accepted = Vec::new();

    'gears: for gear in gears {
        let mut workflow_label = "in";
        'workflow: loop {
            if workflow_label == "A" {
                accepted.push(gear.clone());
                continue 'gears;
            }

            if workflow_label == "R" {
                continue 'gears;
            }

            let rules = workflow.get(workflow_label).unwrap();

            for rule in rules {
                match rule {
                    Accept => {
                        accepted.push(gear.clone());
                        continue 'gears;
                    }
                    Reject => {
                        continue 'gears;
                    }
                    GOTO(label) => {
                        workflow_label = label;
                        continue 'workflow;
                    }
                    GREATER(property, value, label) => {
                        if gear[property] > *value {
                            workflow_label = label;
                            continue 'workflow;
                        }
                    }
                    LESS(property, value, label) => {
                        if gear[property] < *value {
                            workflow_label = label;
                            continue 'workflow;
                        }
                    }
                }
            }
        }
    }

    Some(
        accepted
            .iter()
            .map(|entry| entry.values().sum::<u64>())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (workflow, _) = parse(input.trim_end());

    let mut stack = Vec::new();
    let mut accepted = Vec::new();

    stack.push(((1, 4000), (1, 4000), (1, 4000), (1, 4000), "in", 0));

    while let Some(state) = stack.pop() {
        let (x, m, a, s, workflow_label, rule_key) = state;

        // check if at the end of a workflow chain
        if workflow_label == "A" {
            accepted.push((x, m, a, s));
            continue;
        } else if workflow_label == "R" {
            continue;
        }

        // ensure ranges haven't crossed over
        if x.0 > x.1 || m.0 > m.1 || a.0 > a.1 || s.0 > s.1 {
            continue;
        }

        let rules = workflow.get(workflow_label).unwrap();
        let rule = rules[rule_key];

        match rule {
            Accept => {
                accepted.push((x, m, a, s));
                continue;
            }
            Reject => {
                continue;
            }
            GOTO(label) => {
                stack.push((x, m, a, s, label, 0));
            }
            GREATER(property, value, label) => match property {
                "x" => {
                    stack.push(((value + 1, x.1), m, a, s, label, 0));
                    stack.push(((x.0, value), m, a, s, workflow_label, rule_key + 1))
                }
                "m" => {
                    stack.push((x, (value + 1, m.1), a, s, label, 0));
                    stack.push((x, (m.0, value), a, s, workflow_label, rule_key + 1));
                }
                "a" => {
                    stack.push((x, m, (value + 1, a.1), s, label, 0));
                    stack.push((x, m, (a.0, value), s, workflow_label, rule_key + 1));
                }
                "s" => {
                    stack.push((x, m, a, (value + 1, s.1), label, 0));
                    stack.push((x, m, a, (s.0, value), workflow_label, rule_key + 1));
                }
                _ => (),
            },
            LESS(property, value, label) => match property {
                "x" => {
                    stack.push(((x.0, value - 1), m, a, s, label, 0));
                    stack.push(((value, x.1), m, a, s, workflow_label, rule_key + 1));
                }
                "m" => {
                    stack.push((x, (m.0, value - 1), a, s, label, 0));
                    stack.push((x, (value, m.1), a, s, workflow_label, rule_key + 1));
                }
                "a" => {
                    stack.push((x, m, (a.0, value - 1), s, label, 0));
                    stack.push((x, m, (value, a.1), s, workflow_label, rule_key + 1));
                }
                "s" => {
                    stack.push((x, m, a, (s.0, value - 1), label, 0));
                    stack.push((x, m, a, (value, s.1), workflow_label, rule_key + 1));
                }
                _ => (),
            },
        }
    }

    Some(
        accepted
            .iter()
            .map(|(x, m, a, s)| {
                (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1)
            })
            .sum(),
    )
}

advent_of_code::main!(19);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 19));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 19));
        assert_eq!(result, Some(167409079868000));
    }
}
