use itertools::Itertools;
use std::ops::Sub;

const MIN: f64 = 200000000000000f64;
const MAX: f64 = 400000000000000f64;

// Euclid was a genius so let's use that
fn gcd(first: i128, second: i128) -> i128 {
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

#[derive(Debug)]
struct Hailstone {
    point: (f64, f64, f64),  // (x, y, z)
    vector: (f64, f64, f64), // (dx, dy, dz)
}

impl Hailstone {
    fn new(input: &str) -> Self {
        let (point, vector) = input.split_once(" @ ").expect("Delimited by @");
        let vector: (f64, f64, f64) = vector
            .split(",")
            .map(|num| num.trim().parse().expect("comma delimited numbers"))
            .collect_tuple()
            .expect("three sets of numbers");

        let point = point
            .split(",")
            .map(|num| num.trim().parse().expect("comma delimited numbers"))
            .collect_tuple()
            .expect("three sets of numbers");

        Self { point, vector }
    }
}

type Vector3 = nalgebra::Vector3<i128>;

#[derive(Debug, Copy, Clone)]
struct Line {
    point: Vector3,
    vector: Vector3,
}

impl Sub for &Line {
    type Output = Line;

    fn sub(self, rhs: Self) -> Self::Output {
        Line {
            point: self.point - rhs.point,
            vector: self.vector - rhs.vector,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Plane {
    normal: Vector3,
    constant: i128,
}

impl Plane {
    fn intersect(a: &Self, b: &Self, c: &Self) -> Option<Vector3> {
        let denominator = a.normal.dot(&b.normal.cross(&c.normal));
        // if the denominator is 0 then all vectors involved are non intersecting (Shouldn't happen for this problem set)
        if denominator == 0 {
            return None;
        }

        Some(
            (b.normal.cross(&c.normal) * a.constant
                + c.normal.cross(&a.normal) * b.constant
                + a.normal.cross(&b.normal) * c.constant)
                / denominator,
        )
    }

    fn reduce(&mut self) {
        // these numbers are huge and prone to overflowing, try to keep them manageable
        let gcd = gcd(
            gcd(gcd(self.constant, self.normal.x), self.normal.y),
            self.normal.z,
        );
        self.normal /= gcd;
        self.constant /= gcd;
    }
}

fn parse_vector3(input: &str) -> Vector3 {
    Vector3::from_iterator(input.split(", ").map(|coord| coord.trim().parse().unwrap()))
}

// Construct plane of velocities that would hit both hailstones: i.e. their intersection plane
fn find_rock_velocities(a: &Line, b: &Line) -> Plane {
    // simplify frame of reference to have b stationary at the origin
    // and a is moving relative to b
    let relative_a = a - b;

    // in order for a rock to hit both a and b hailstones
    // it must be moving on a plane formed by the following three points:
    // the position of hailstone b (origin point),
    // relative position of hailstone a,
    // and the relative position of hailstone a after one step

    // normal of a plane is perpendicular to any two vectors between those points
    let normal = relative_a.point.cross(&relative_a.vector);

    // convert our coordinates from relative to absolute
    // plane will still have the correct orientation but will probably not go through the origin anymore
    // we can adjust back to this relative origin via a constant on the relative plane

    // calculate the velocity of a rock that starts at b and hits a after one step
    let rock_velocity = relative_a.point + a.vector;
    let constant = rock_velocity.dot(&normal);

    let mut plane = Plane { normal, constant };

    // try to reduce the numbers down to keep them manageable
    plane.reduce();

    return plane;
}

fn intersect(hailstone1: &Hailstone, hailstone2: &Hailstone) -> Option<(f64, f64)> {
    let m1 = hailstone1.vector.1 / hailstone1.vector.0;
    let m2 = hailstone2.vector.1 / hailstone2.vector.0;
    if (m2 - m1).abs() < f64::EPSILON {
        // equal slopes means parallel lines
        return None;
    }
    let x = (m1 * hailstone1.point.0 - m2 * hailstone2.point.0 + hailstone2.point.1
        - hailstone1.point.1)
        / (m1 - m2);
    let y = (m1 * m2 * (hailstone2.point.0 - hailstone1.point.0) + m2 * hailstone1.point.1
        - m1 * hailstone2.point.1)
        / (m2 - m1);

    // check if happened in the past
    if (x - hailstone1.point.0) / hailstone1.vector.0 > 0.0
        && (x - hailstone2.point.0) / hailstone2.vector.0 > 0.0
    {
        return Some((x, y));
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let hailstones: Vec<Hailstone> = input.trim_end().lines().map(Hailstone::new).collect();

    let mut count = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            match intersect(&hailstones[i], &hailstones[j]) {
                Some((x, y)) => {
                    if x >= MIN && x <= MAX && y >= MIN && y <= MAX {
                        count += 1;
                    }
                }
                None => (),
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<i128> {
    let hailstones = input
        .trim_end()
        .lines()
        .map(|line| {
            let (point, vector) = line
                .split(" @ ")
                .map(parse_vector3)
                .collect_tuple()
                .unwrap();
            Line { point, vector }
        })
        .collect_vec();

    // Compute the rock velocity

    // construct three planes of velocity from our first three independent hailstones
    let a = find_rock_velocities(&hailstones[0], &hailstones[1]);
    let b = find_rock_velocities(&hailstones[0], &hailstones[2]);
    let c = find_rock_velocities(&hailstones[1], &hailstones[2]);

    // intersection of all three planes will be the line common through the given planes, thus our desired velocity
    let rock_velocity = Plane::intersect(&a, &b, &c)
        .expect("All rays should have an intersection given our problem constraint.");

    // now work backwards to find the starting position
    // treat the rock as stationary and the velocity of a hailstone line is '-rock_velocity'
    // then find where our adjusted lines intersect to find our starting point (we only need two for a single point)
    let rock = Line {
        point: Vector3::zeros(),
        vector: rock_velocity,
    };

    let hailstone_a = &hailstones[0] - &rock;
    let hailstone_b = &hailstones[1] - &rock;

    // line intersection
    // find out how far along hailstone_a we hit hailstone_b by finding the difference of the points
    // and dividing how quickly hailstone_a approaches hailstone_b. We nullify the impact of
    // hailstone_b's velocity by taking the cross product of both sides

    let top = (hailstone_b.point - hailstone_a.point).cross(&hailstone_b.vector);
    let bottom = hailstone_a.vector.cross(&hailstone_b.vector);

    // vectors are parallel so it doesn't matter which non-zero component we take for division
    let t0 = top.x / bottom.x;

    // Calculate the rock's position at t0

    let rock_position = hailstone_a.point + hailstone_a.vector * t0;

    Some(rock_position.x + rock_position.y + rock_position.z)
}

advent_of_code::main!(24);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 24));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 24));
        assert_eq!(result, Some(47));
    }
}
