use std::fmt;

use {lazy_static::lazy_static, regex::Regex};

#[derive(Debug, Clone, Copy)]
struct Point(usize, usize);

#[derive(Debug, Clone, Copy)]
struct Line(Point, Point);

impl Line {
    fn is_straight(&self) -> bool {
        let Line(Point(x1, y1), Point(x2, y2)) = self;

        x1 == x2 || y1 == y2
    }
}

#[derive(Clone, Copy)]
struct Space<const SIZE: usize>([[u32; SIZE]; SIZE]);

impl<const SIZE: usize> Space<SIZE> {
    fn new() -> Self {
        Space([[0; SIZE]; SIZE])
    }

    fn apply_line(&mut self, line: &Line) {
        let Line(Point(x1, y1), Point(x2, y2)) = line;
        assert!(*x1 < SIZE);
        assert!(*x2 < SIZE);
        assert!(*y1 < SIZE);
        assert!(*y2 < SIZE);

        if y1 == y2 {
            self.apply_horiz_line(*y1, *x1, *x2);
        } else if x1 == x2 {
            self.apply_vert_line(*x1, *y1, *y2);
        } else {
            self.apply_diag_line(line.0, line.1);
        }
    }

    fn apply_horiz_line(&mut self, y: usize, x1: usize, x2: usize) {
        let range = if x1 < x2 { x1..=x2 } else { x2..=x1 };
        for cell in &mut self.0[y][range] {
            *cell += 1;
        }
    }

    fn apply_vert_line(&mut self, x: usize, y1: usize, y2: usize) {
        let range = if y1 < y2 { y1..=y2 } else { y2..=y1 };
        for row in &mut self.0[range] {
            row[x] += 1;
        }
    }

    fn apply_diag_line(&mut self, Point(x1, y1): Point, Point(x2, y2): Point) {
        for (x, y) in make_range(x1, x2).zip(make_range(y1, y2)) {
            self.0[y][x] += 1;
        }
    }

    fn count_intersenctions(&self) -> usize {
        self.0.iter().flatten().filter(|c| **c > 1).count()
    }
}

pub fn part1(input: String) {
    let lines: Vec<_> = parse_input(&input)
        .into_iter()
        .filter(Line::is_straight)
        .collect();

    let mut space = Space::<1000>::new();
    for line in lines {
        space.apply_line(&line);
    }

    println!("Intersections: {}", space.count_intersenctions());
}

pub fn part2(input: String) {
    let lines: Vec<_> = parse_input(&input);

    let mut space = Space::<1000>::new();
    for line in lines {
        space.apply_line(&line);
    }

    println!("Intersections: {}", space.count_intersenctions());
}

fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Line {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d{1,3}),(\d{1,3}) -> (\d{1,3}),(\d{1,3})").unwrap();
    }

    let captures = RE.captures(line).unwrap();

    Line(
        Point(
            captures.get(1).unwrap().as_str().parse().unwrap(),
            captures.get(2).unwrap().as_str().parse().unwrap(),
        ),
        Point(
            captures.get(3).unwrap().as_str().parse().unwrap(),
            captures.get(4).unwrap().as_str().parse().unwrap(),
        ),
    )
}

fn make_range(a: usize, b: usize) -> Box<dyn Iterator<Item = usize>> {
    if a < b {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Line(Point(x1, y1), Point(x2, y2)) = self;
        write!(f, "({}, {}) -> ({}, {})", x1, y1, x2, y2)
    }
}

impl<const SIZE: usize> fmt::Display for Space<SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.0 {
            let line_fmt = line
                .iter()
                .map(|c| match c {
                    0 => ".".to_string(),
                    n => n.to_string(),
                })
                .collect::<String>();
            writeln!(f, "{}", line_fmt)?;
        }
        Ok(())
    }
}
