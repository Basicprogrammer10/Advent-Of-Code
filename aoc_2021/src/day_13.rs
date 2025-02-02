use hashbrown::HashSet;

use common::{solution, Answer};
use nd_vec::vector;

type Point = nd_vec::Vec2<usize>;

solution!("Transparent Origami", 13);

fn part_a(input: &str) -> Answer {
    let mut paper = Paper::parse(input);
    paper.fold(0);

    paper.data.len().into()
}

fn part_b(input: &str) -> Answer {
    let mut paper = Paper::parse(input);
    (0..paper.folds.len()).for_each(|x| paper.fold(x));

    paper.print().into()
}

#[derive(Debug)]
struct Paper {
    data: HashSet<Point>,
    folds: Vec<Fold>,
}

#[derive(Debug)]
struct Fold {
    direction: Direction,
    position: usize,
}

#[derive(Debug)]
enum Direction {
    Horizontal,
    Vertical,
}

impl Paper {
    fn parse(raw: &str) -> Self {
        let mut parts = raw.split("\n\n");
        let data = parts.next().unwrap().lines().map(parse_point).collect();
        let folds = parts.next().unwrap().lines().map(parse_fold).collect();

        Self { data, folds }
    }

    // Cordantes go from 0 onwards
    fn fold(&mut self, ins: usize) {
        let ins = &self.folds[ins];

        match ins.direction {
            Direction::Horizontal => {
                for i in self.data.clone().iter().filter(|x| x.x() > ins.position) {
                    self.data.remove(i);
                    self.data.insert(vector!(ins.position * 2 - i.x(), i.y()));
                }
            }
            Direction::Vertical => {
                for i in self.data.clone().iter().filter(|x| x.y() > ins.position) {
                    self.data.remove(i);
                    self.data.insert(vector!(i.x(), ins.position * 2 - i.y()));
                }
            }
        }
    }

    fn bounds(&self) -> (usize, usize) {
        let x = self.data.iter().map(|x| x.x()).max().unwrap();
        let y = self.data.iter().map(|x| x.y()).max().unwrap();
        (x, y)
    }

    fn print(&self) -> String {
        let (mx, my) = self.bounds();
        let mut out = "\n".to_owned();

        for y in 0..=my {
            for x in 0..=mx {
                let point = vector!(x, y);
                if self.data.contains(&point) {
                    out.push('#');
                } else {
                    out.push(' ');
                }
            }
            out.push('\n');
        }

        out
    }
}

fn parse_point(raw: &str) -> Point {
    let parts = raw.split_once(',').unwrap();
    let x = parts.0.parse().unwrap();
    let y = parts.1.parse().unwrap();
    vector!(x, y)
}

fn parse_fold(raw: &str) -> Fold {
    let parts = raw.rsplit_once(' ').unwrap().1.split_once('=').unwrap();
    let position = parts.1.parse().unwrap();
    let direction = match parts.0 {
        "x" => Direction::Horizontal,
        "y" => Direction::Vertical,
        _ => unreachable!(),
    };

    Fold {
        direction,
        position,
    }
}
