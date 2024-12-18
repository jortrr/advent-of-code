use crate::*;

type Almanac = Vec<Vec<Transform>>;

#[derive(Clone)]
struct Interval {
    a: Int,
    b: Int,
    v: bool,
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl Debug for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interval")
            .field("a", &self.a)
            .field("b", &self.b)
            .finish()
    }
}

impl Interval {
    fn new(a: Int, b: Int) -> Interval {
        assert!(
            a <= b,
            "Invalid interval: a ({}) <= b ({}) must hold.",
            a,
            b
        );
        Interval { a, b, v: false }
    }

    fn len(&self) -> Int {
        self.b - self.a
    }

    fn split(&mut self, at: Int) -> Interval {
        assert!(at >= self.a && at <= self.b);
        let split_off_part = Interval::new(at, self.b);
        self.b = at - 1;
        split_off_part
    }

    fn shift(&mut self, by: Int) -> &mut Interval {
        self.a += by;
        self.b += by;
        self
    }

    fn apply(&self, t: &Transform) -> Vec<Interval> {
        let mut result: Vec<Interval> = vec![self.clone()];
        let mut transformed = false;

        if self.a >= t.source.a && self.b <= t.source.b {
            result[0].shift(t.get_shift());
            transformed = true;
        } else {
            let split_by_source_a = t.source.a > self.a && t.source.a <= self.b;
            let split_by_source_b = t.source.b < self.b && t.source.b >= self.a;

            if split_by_source_a {
                let mut first = result.pop().unwrap();
                let second = first.split(t.source.a);
                result = vec![first, second];
                transformed = true;
            }
            if split_by_source_b {
                let mut last = result.pop().unwrap();
                let second = last.split(t.source.b);
                result.push(last);
                result.push(second);
                transformed = true;
            }
            if split_by_source_a && split_by_source_b {
                result[1].shift(t.get_shift());
            } else if split_by_source_a {
                result[1].shift(t.get_shift());
            } else if split_by_source_b {
                result[0].shift(t.get_shift());
            }
        }
        if transformed {
            for i in &mut result {
                i.v = true;
            }
        }

        result
    }
}

#[derive(Debug)]
struct Transform {
    source: Interval,
    destination: Interval,
}

impl Transform {
    fn new(destination: Int, source: Int, length: Int) -> Transform {
        Transform {
            source: Interval::new(source, source + length),
            destination: Interval::new(destination, destination + length),
        }
    }

    fn get_shift(&self) -> Int {
        assert!(self.source.len() == self.destination.len());
        self.destination.a - self.source.a
    }

    fn apply(&self, intervals: Vec<Interval>) -> Vec<Interval> {
        let mut result: Vec<Interval> = Vec::new();
        for interval in intervals {
            if !interval.v {
                let new_intervals = interval.apply(self);
                result = result
                    .into_iter()
                    .chain(new_intervals.into_iter())
                    .collect();
            } else {
                result.push(interval);
            }
        }
        result
    }
}

fn apply_almanac(s: Vec<Interval>, a: &Almanac) -> Vec<Interval> {
    let mut s = s;
    for (i, transforms) in a.iter().enumerate() {
        for t in transforms {
            let old_s = s.clone();
            s = t.apply(s);
            if old_s != s {
                debug!(false, "({:?},{:?},{:?})", old_s, t, &s);
            }
        }
        debug!(false, "[{}]: {:?}", i, &s);
        for i in &mut s {
            i.v = false;
        }
    }
    s
}

fn parse(e: &Vec<String>, seeds_is_range: bool) -> (Vec<Interval>, Almanac) {
    let mut s: Vec<Interval> = Vec::new();
    let mut a: Almanac = Vec::new();
    a.push(Vec::new());
    let mut c = 0;
    for l in e {
        if l.contains("seeds:") {
            let i: Vec<_> = l[6..]
                .split_whitespace()
                .map(|x| x.parse::<Int>().unwrap())
                .collect();
            if seeds_is_range {
                for x in 0..i.len() {
                    if x % 2 == 1 {
                        continue;
                    }
                    s.push(Interval::new(i[x], i[x] + i[x + 1] - 1));
                }
            } else {
                for x in i {
                    s.push(Interval::new(x, x));
                }
            }
        } else if l.contains("map:") {
            a.push(Vec::new());
            if !a[c].is_empty() {
                c += 1;
            }
        } else if l.is_empty() {
            continue;
        } else {
            let i: Vec<_> = l
                .split_whitespace()
                .map(|x| x.parse::<Int>().unwrap())
                .collect();

            a[c].push(Transform::new(i[0], i[1], i[2]));
        }
    }
    (s, a)
}

#[derive(Default)]
pub struct Problem {}

impl Solution for Problem {
    fn year(&self) -> Year {
        2023
    }
    fn day(&self) -> Day {
        5
    }
    fn expect_part_one(&self) -> Answer {
        251346198
    }
    fn expect_part_two(&self) -> Answer {
        72263011
    }

    define_examples! {
        (
            "
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4

            ",
            Expect::PartsOneAndTwo(35, 46),
        )
    }

    fn solve_part_one(&self, input: Input, _is_example: bool) -> Answer {
        let input = input.lines().map(|s| s.to_string()).collect(); // Todo: Make Input convertible to Vec<String>, and vice versa
        let (s, a) = parse(&input, false);
        let mut s = apply_almanac(s, &a);
        s.sort_by_key(|i| i.a);
        s[0].a
    }

    fn solve_part_two(&self, input: Input, _is_example: bool) -> Answer {
        let input = input.lines().map(|s| s.to_string()).collect();
        let (s, a) = parse(&input, true);
        let mut s = apply_almanac(s, &a);
        s.sort_by_key(|i| i.a);
        s[0].a
    }
}
