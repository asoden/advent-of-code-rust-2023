use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Brick {
    x1: u32,
    y1: u32,
    z1: u32,
    x2: u32,
    y2: u32,
    z2: u32,
    id: usize,
    bricks_below: Vec<usize>,
    bricks_above: Vec<usize>,
}

impl Brick {
    fn new(line: &str) -> Self {
        let coords: Vec<_> = line
            .split(|c| c == ',' || c == '~')
            .map(|num| num.parse().unwrap())
            .collect();

        Self {
            x1: coords[0],
            y1: coords[1],
            z1: coords[2],
            x2: coords[3],
            y2: coords[4],
            z2: coords[5],
            id: 0,
            bricks_below: Vec::with_capacity(10),
            bricks_above: Vec::with_capacity(10),
        }
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn lowest_z(&self) -> u32 {
        self.z1.min(self.z2)
    }

    fn highest_z(&self) -> u32 {
        self.z1.max(self.z2)
    }

    fn can_remove(&self, bricks: &[Brick]) -> bool {
        for &brick in &self.bricks_above {
            if bricks[brick].bricks_below.len() == 1 {
                return false;
            }
        }
        return true;
    }

    fn will_fall(&self, falling: &[bool]) -> bool {
        self.bricks_below.iter().all(|&brick| falling[brick])
    }

    fn drop(&mut self, height_shadow: &mut [[Option<usize>; 10]; 10], bricks: &mut Vec<Brick>) {
        let mut below = Vec::new();
        let mut new_height = 0;

        for x in self.x1 as usize..=self.x2 as usize {
            for y in self.y1 as usize..=self.y2 as usize {
                if height_shadow[x][y] != None && !below.contains(&height_shadow[x][y].unwrap()) {
                    let id = height_shadow[x][y].unwrap();
                    new_height = new_height.max(bricks[id].highest_z());
                    below.push(id);
                }
                height_shadow[x][y] = Some(self.id);
            }
        }

        // update the references for surrounding bricks
        for id in below {
            if bricks[id].highest_z() == new_height {
                self.bricks_below.push(id);
                bricks[id].bricks_above.push(self.id);
            }
        }

        if self.z1 < self.z2 {
            let d = self.z2 - self.z1;
            self.z1 = new_height + 1;
            self.z2 = self.z1 + d;
        } else {
            let d = self.z1 - self.z2;
            self.z2 = new_height + 1;
            self.z1 = self.z2 + d;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut bricks: Vec<Brick> = input.trim_end().lines().map(Brick::new).collect();

    let mut height_shadow: [[Option<usize>; 10]; 10] = [[None; 10]; 10];

    // sort bricks by initial lowest z position
    bricks.sort_by_key(|brick| brick.lowest_z());
    bricks
        .iter_mut()
        .enumerate()
        .for_each(|(i, brick)| brick.set_id(i));

    //drop the bricks
    for i in 0..bricks.len() {
        let mut brick = bricks[i].clone();
        brick.drop(&mut height_shadow, &mut bricks);
        bricks[i] = brick;
    }

    let mut count = 0;
    for i in 0..bricks.len() {
        if bricks[i].can_remove(&bricks) {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bricks: Vec<Brick> = input.trim_end().lines().map(Brick::new).collect();

    let mut height_shadow: [[Option<usize>; 10]; 10] = [[None; 10]; 10];

    // sort bricks by initial lowest z position
    bricks.sort_by_key(|brick| brick.lowest_z());
    bricks
        .iter_mut()
        .enumerate()
        .for_each(|(i, brick)| brick.set_id(i));

    //drop the bricks
    for i in 0..bricks.len() {
        let mut brick = bricks[i].clone();
        brick.drop(&mut height_shadow, &mut bricks);
        bricks[i] = brick;
    }

    let mut falling_bricks = vec![false; bricks.len()];
    let mut queue = VecDeque::new();
    let mut count = 0;

    for brick in &bricks {
        // set brick to falling to try starting a chain reaction
        falling_bricks[brick.id] = true;

        queue.push_back(brick.id);

        // search up the tree via bricks above our current
        while let Some(brick) = queue.pop_front() {
            for above_brick in &bricks[brick].bricks_above {
                if !falling_bricks[*above_brick] && bricks[*above_brick].will_fall(&falling_bricks)
                {
                    falling_bricks[*above_brick] = true;
                    queue.push_back(*above_brick);
                    count += 1;
                }
            }
        }

        // reset falling status
        falling_bricks.fill(false);
    }

    Some(count)
}

advent_of_code::main!(22);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 22));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 22));
        assert_eq!(result, Some(7));
    }
}
