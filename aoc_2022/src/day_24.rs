use common::{Answer, Solution};
use hashbrown::HashSet;
use nd_vec::{vector, Vec2};
use pathfinding::directed::bfs::bfs;

type Pos = Vec2<usize>;

pub struct Day24;

impl Solution for Day24 {
    fn name(&self) -> &'static str {
        "Blizzard Basin"
    }

    fn part_a(&self, input: &str) -> Answer {
        let basin = Basin::parse(input);
        let end = basin.end();
        let states = basin.all_states();

        let path = bfs(
            &(vector!(1, 0), 0),
            move |(pos, idx)| {
                states[(idx + 1) % states.len()]
                    .available(*pos)
                    .iter()
                    .map(move |x| (*x, *idx))
                    .collect::<Vec<_>>()
            },
            |(pos, _)| *pos == end,
        )
        .unwrap();

        path.len().into()
    }

    fn part_b(&self, _input: &str) -> Answer {
        Answer::Unimplemented
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// [up, down, left, right]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Blizzard(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
    Blizzard(Blizzard),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Basin {
    tiles: Vec<Vec<Tile>>,
    size: Pos,
}

impl Basin {
    fn parse(input: &str) -> Self {
        let mut tiles = Vec::new();

        for row in input.lines() {
            tiles.push(row.trim().chars().map(Tile::from_char).collect::<Vec<_>>())
        }

        Basin {
            size: vector!(tiles[0].len(), tiles.len()),
            tiles,
        }
    }

    fn available(&self, pos: Pos) -> Vec<Pos> {
        let mut out = Vec::new();
        let [x, y] = [pos.x(), pos.y()];
        let [sx, sy] = [self.size.x(), self.size.y()];

        if self.tiles[y][x] == Tile::Empty {
            out.push(vector!(x, y));
        }

        if x >= 1 && self.tiles[y][x - 1] == Tile::Empty {
            out.push(vector!(x - 1, y));
        }

        if x + 1 < sx && self.tiles[y][x + 1] == Tile::Empty {
            out.push(vector!(x + 1, y));
        }

        if y + 1 < sy && self.tiles[y + 1][x] == Tile::Empty {
            out.push(vector!(x, y + 1));
        }

        if y >= 1 && self.tiles[y - 1][x] == Tile::Empty {
            out.push(vector!(x, y - 1));
        }

        out
    }

    fn tick(&self) -> Self {
        let mut tiles = vec![vec![Tile::Empty; self.size.x()]; self.size.y()];

        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Empty => {}
                    Tile::Wall => tiles[y][x] = Tile::Wall,
                    Tile::Blizzard(blizzard) => {
                        let pos = vector!(x, y);
                        for dir in blizzard.directions() {
                            let mut new_pos = dir.advance(pos);

                            let [nx, ny] = [new_pos.x(), new_pos.y()];
                            if self.tiles[ny][nx] == Tile::Wall {
                                let [sx, sy] = [self.size.x() - 2, self.size.y() - 2];
                                new_pos = match dir {
                                    Direction::Up => vector!(nx, sy),
                                    Direction::Down => vector!(nx, 1),
                                    Direction::Left => vector!(sx, ny),
                                    Direction::Right => vector!(1, ny),
                                }
                            }

                            let new_blizzard = dir.as_blizzard();
                            let [nx, ny] = [new_pos.x(), new_pos.y()];
                            match tiles[ny][nx] {
                                Tile::Empty => tiles[ny][nx] = Tile::Blizzard(new_blizzard),
                                Tile::Blizzard(blizzard) => {
                                    tiles[ny][nx] = Tile::Blizzard(blizzard.or(new_blizzard))
                                }
                                Tile::Wall => unreachable!(),
                            }
                        }
                    }
                }
            }
        }

        Self { tiles, ..*self }
    }

    fn all_states(mut self) -> Vec<Self> {
        let mut seen = HashSet::new();
        let mut out = Vec::new();

        while seen.insert(self.clone()) {
            out.push(self.clone());
            self = self.tick();
        }

        out
    }

    fn end(&self) -> Pos {
        vector!(self.size.y() - 1, self.size.x() - 2)
    }

    fn print_board(&self) {
        for row in &self.tiles {
            for tile in row {
                print!(
                    "{}",
                    match tile {
                        Tile::Empty => '.',
                        Tile::Wall => '#',
                        Tile::Blizzard(b) => {
                            let dir = b.directions();
                            if dir.len() == 1 {
                                match dir[0] {
                                    Direction::Up => '^',
                                    Direction::Down => 'v',
                                    Direction::Left => '<',
                                    Direction::Right => '>',
                                }
                            } else {
                                dir.len().to_string().chars().next().unwrap()
                            }
                        }
                    }
                )
            }
            println!();
        }
    }
}

impl Direction {
    fn as_blizzard(&self) -> Blizzard {
        match self {
            Direction::Up => Blizzard(0b1000),
            Direction::Down => Blizzard(0b0100),
            Direction::Left => Blizzard(0b0010),
            Direction::Right => Blizzard(0b0001),
        }
    }

    fn advance(&self, pos: Pos) -> Pos {
        match self {
            Direction::Up => pos - vector!(0, 1),
            Direction::Down => pos + vector!(0, 1),
            Direction::Left => pos - vector!(1, 0),
            Direction::Right => pos + vector!(1, 0),
        }
    }
}

impl Blizzard {
    fn directions(&self) -> Vec<Direction> {
        let mut out = Vec::new();

        for (mask, direction) in [
            (0b1000, Direction::Up),
            (0b0100, Direction::Down),
            (0b0010, Direction::Left),
            (0b0001, Direction::Right),
        ] {
            if self.0 & mask != 0 {
                out.push(direction);
            }
        }

        out
    }

    fn and(&self, other: Blizzard) -> Self {
        Blizzard(self.0 & other.0)
    }

    fn or(&self, other: Blizzard) -> Self {
        Blizzard(self.0 | other.0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl Tile {
    fn from_char(input: char) -> Self {
        match input {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            '^' => Tile::Blizzard(Direction::Up.as_blizzard()),
            'v' => Tile::Blizzard(Direction::Down.as_blizzard()),
            '<' => Tile::Blizzard(Direction::Left.as_blizzard()),
            '>' => Tile::Blizzard(Direction::Right.as_blizzard()),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use crate::day_24::{Basin, Blizzard};

    use super::Day24;

    const CASE: &str = indoc! {"
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day24.part_a(CASE), 18.into());
    }

    #[test]
    fn tick() {
        const CASE: &str = indoc! {"
            #.#####
            #.....#
            #>....#
            #.....#
            #...v.#
            #.....#
            #####.#
        "};

        let mut bliz = Basin::parse(CASE);
        bliz.print_board();
        println!();

        for _ in 0..3 {
            bliz = bliz.tick();
            bliz.print_board();
            println!();
        }
    }
}
