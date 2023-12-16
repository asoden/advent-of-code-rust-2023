#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Box<'a> {
    label: &'a str,
    lens: u32,
}

fn hash(input: &[u8]) -> u32 {
    input.iter().fold(0_u16, |mut val, x| {
        val += *x as u16;
        val *= 17;
        val %= 256;
        val
    }) as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim_end()
            .split(",")
            .into_iter()
            .map(str::as_bytes)
            .map(hash)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut boxes: Vec<Vec<Box>> = vec![Vec::new(); 256];

    input.trim_end().split(",").into_iter().for_each(|entry| {
        let character_index = entry.find(|x| x == '=' || x == '-').expect("Label = or -");
        let label = entry.get(0..character_index).expect("Label");
        let operation = entry.chars().nth(character_index).expect("Command symbol");
        let num = entry.get(character_index + 1..);

        let index = hash(label.as_bytes());

        match operation {
            '=' => {
                let new_box = Box {
                    label,
                    lens: num.unwrap().parse().unwrap(),
                };

                let mut found = false;
                let items = &mut boxes[index as usize];
                for i in 0..items.len() {
                    if items[i].label == new_box.label {
                        items[i].lens = new_box.lens;
                        found = true;
                        break;
                    }
                }

                if !found {
                    items.push(new_box);
                }
            }
            '-' => {
                let mut to_remove = -1;
                let items = &mut boxes[index as usize];
                for i in 0..items.len() {
                    if items[i].label == label {
                        to_remove = i as i64;
                    }
                }

                if to_remove >= 0 {
                    items.remove(to_remove as usize);
                }
            }
            _ => (),
        }
    });

    Some(
        boxes
            .iter()
            .enumerate()
            .map(|(box_index, boxes)| {
                let mut total = 0;
                for (i, x) in boxes.iter().enumerate() {
                    total += (box_index + 1) as u64 * (i + 1) as u64 * x.lens as u64;
                }
                total
            })
            .sum(),
    )
}

advent_of_code::main!(15);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 15));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 15));
        assert_eq!(result, Some(145));
    }
}
