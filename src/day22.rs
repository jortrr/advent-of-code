mod line_segment;
mod problem;
use std::cmp::{max, min};

use line_segment::LineSegment;
use problem::*;

type BrickID = Int;
type Bricks = HashMap<BrickID, Brick>;

#[derive(Debug)]
struct Point {
    x: Int,
    y: Int,
    z: Int,
}

impl Point {
    fn translate(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

struct Brick {
    id: BrickID,
    head: Point,
    tail: Point,
    support: Vec<BrickID>,
}

impl Parse for Brick {
    fn parse(input: Input) -> Self {
        Brick::parse_result(&input).unwrap().1
    }
}

impl Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}~{},{},{}  <- {} {:?}",
            self.head.x,
            self.head.y,
            self.head.z,
            self.tail.x,
            self.tail.y,
            self.tail.z,
            self.id,
            self.support
        )
    }
}

impl Brick {
    fn parse_bricks(input: Input) -> Bricks {
        input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                (
                    i as BrickID,
                    Brick::parse(l.to_string()).assign_id(i as BrickID),
                )
            })
            .collect()
    }

    fn parse_result(input: &str) -> IResult<&str, Brick> {
        let parse_point = |input| {
            let (rest, (x, y, z)) = tuple((
                terminated(parse_num, tag(",")),
                terminated(parse_num, tag(",")),
                parse_num,
            ))(input)?;
            Ok((rest, Point { x, y, z }))
        };
        let (rest, head) = parse_point(input)?;
        let (rest, _) = tag("~")(rest)?;
        let (rest, tail) = parse_point(rest)?;
        Ok((
            rest,
            Brick {
                id: -1,
                head,
                tail,
                support: Vec::new(),
            },
        ))
    }

    fn intersects_in_xy_plane(&self, other: &Brick) -> bool {
        line_segment::intersects(
            &LineSegment::new(self.head.x, self.head.y, self.tail.x, self.tail.y),
            &LineSegment::new(other.head.x, other.head.y, other.tail.x, other.tail.y),
        )
    }

    fn assign_id(mut self, id: BrickID) -> Brick {
        self.id = id;
        self
    }

    /// Get the minimum value of the bricks in dimension ('x', 'y', 'x')
    fn get_min(&self, dimension: char) -> Int {
        match dimension {
            'x' => min(self.head.x, self.tail.x),
            'y' => min(self.head.y, self.tail.y),
            'z' => min(self.head.z, self.tail.z),
            _ => unreachable!(),
        }
    }

    /// Get the max value of the bricks in dimension ('x', 'y', 'x')
    fn get_max(&self, dimension: char) -> Int {
        match dimension {
            'x' => max(self.head.x, self.tail.x),
            'y' => max(self.head.y, self.tail.y),
            'z' => max(self.head.z, self.tail.z),
            _ => unreachable!(),
        }
    }

    fn is_falling(&self) -> bool {
        self.get_min('z') > 1 && self.support.is_empty()
    }

    fn translate(&mut self, point: Point) {
        self.head.translate(&point);
        self.tail.translate(&point);
    }

    fn fall_distance(&mut self, distance: Int) {
        self.translate(Point {
            x: 0,
            y: 0,
            z: -distance,
        });
    }

    /// Fall until !self.is_falling()
    /// Optimized implementation, look in Git history for a more natural implementation :)
    fn fall(&mut self, bricks: &Queue<&mut Brick>) {
        let self_min_z = self.get_min('z');
        let lower_bricks: Vec<_> = bricks
            .iter()
            .filter(|b| b.get_max('z') < self_min_z)
            .filter(|b| self.intersects_in_xy_plane(b))
            .collect();
        if lower_bricks.is_empty() {
            self.fall_distance(self_min_z - 1);
        } else {
            let other_max_z = lower_bricks.iter().map(|b| b.get_max('z')).max().unwrap();
            self.support = lower_bricks
                .iter()
                .filter(|b| b.get_max('z') == other_max_z)
                .map(|b| b.id)
                .collect();
            self.fall_distance(self_min_z - other_max_z - 1);
        }
    }
}

/// Simulate the falling bricks until they have all found support
fn let_fall(bricks: Bricks) -> Bricks {
    let mut bricks = bricks;
    let mut falling_bricks: Vec<_> = bricks.values_mut().collect();
    falling_bricks.sort_by_key(|b| b.get_min('z'));
    let mut falling_bricks: Queue<&mut Brick> = falling_bricks.into();
    let mut supported_bricks: Queue<&mut Brick> = Queue::new();
    while let Some(brick) = falling_bricks.pop_front() {
        if brick.is_falling() {
            brick.fall(&supported_bricks);
        }
        supported_bricks.push_front(brick);
    }
    bricks
}

struct DayTwentyTwo {}

impl Problem for DayTwentyTwo {
    const YEAR: Year = 2023;
    const DAY: Day = 22;
    const PART_ONE_EXPECTED: Answer = 465;
    const PART_TWO_EXPECTED: Answer = 0;

    define_examples! {
        (
            "
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9
            ",
            Expect::PartOne(5),
        )
    }

    fn solve_part_one(input: Input, is_example: bool) -> Answer {
        let bricks = let_fall(Brick::parse_bricks(input));
        if is_example {
            let mut z_sorted_bricks: Vec<_> = bricks.values().collect();
            z_sorted_bricks.sort_by_key(|b| b.get_min('z'));
            debug!(is_example, z_sorted_bricks);
        }
        bricks
            .values()
            .filter(|b| {
                bricks
                    .values()
                    .filter(|o| o.support.contains(&b.id))
                    .all(|o| o.support.len() > 1)
            })
            .count() as Answer
    }

    fn solve_part_two(input: Input, is_example: bool) -> Answer {
        todo!()
    }
}

run!(DayTwentyTwo);
