trait Position {
    fn apply_movement(&self, movement: Movement) -> Self;
    fn x(&self) -> u32;
    fn z(&self) -> u32;
    fn result(&self) -> u32 {
        self.x() * self.z()
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct NaivePosition {
    x: u32,
    z: u32,
}

impl Position for NaivePosition {
    fn apply_movement(&self, movement: Movement) -> Self {
        match movement.direction {
            MovementDirection::Forward => Self {
                x: self.x + movement.distance,
                z: self.z,
            },
            MovementDirection::Down => Self {
                x: self.x,
                z: self.z + movement.distance,
            },
            MovementDirection::Up => Self {
                x: self.x,
                z: self.z - movement.distance,
            },
        }
    }
    fn x(&self) -> u32 {
        self.x
    }
    fn z(&self) -> u32 {
        self.z
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct ComplexPosition {
    x: u32,
    z: u32,
    aim: u32,
}

impl Position for ComplexPosition {
    fn apply_movement(&self, movement: Movement) -> Self {
        match movement.direction {
            MovementDirection::Forward => Self {
                x: self.x + movement.distance,
                z: self.z + self.aim * movement.distance,
                aim: self.aim,
            },
            MovementDirection::Down => Self {
                x: self.x,
                z: self.z,
                aim: self.aim + movement.distance,
            },
            MovementDirection::Up => Self {
                x: self.x,
                z: self.z,
                aim: self.aim - movement.distance,
            },
        }
    }
    fn x(&self) -> u32 {
        self.x
    }
    fn z(&self) -> u32 {
        self.z
    }
}

#[derive(Debug, Clone, Copy)]
enum MovementDirection {
    Forward,
    Down,
    Up,
}

#[derive(Debug, Clone, Copy)]
struct Movement {
    direction: MovementDirection,
    distance: u32,
}

pub fn part1(input: String) {
    let movements = parse_movements(&input);
    let mut position = NaivePosition::default();
    for movement in movements {
        position = position.apply_movement(movement);
    }

    println!("Part 1 Result: {}", position.result());
}

pub fn part2(input: String) {
    let movements = parse_movements(&input);
    let mut position = ComplexPosition::default();
    for movement in movements {
        position = position.apply_movement(movement);
    }

    println!("Part 2 Result: {}", position.result());
}

fn parse_movements(input: &str) -> Vec<Movement> {
    input
        .lines()
        .map(|line| line.split_once(' ').expect("Invalid line format"))
        .map(|(unparsed_direction, unparsed_distance)| {
            let direction = match unparsed_direction {
                "forward" => MovementDirection::Forward,
                "up" => MovementDirection::Up,
                "down" => MovementDirection::Down,
                _ => panic!("Invalid direction"),
            };
            let distance = unparsed_distance.parse().expect("Invalid distance");
            Movement {
                direction,
                distance,
            }
        })
        .collect()
}
