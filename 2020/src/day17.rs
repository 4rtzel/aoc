use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::cmp::{min, max};

#[derive(Clone)]
struct InfiniteGrid {
    grid: HashMap<(i32, i32, i32, i32), bool>,
    min: (i32, i32, i32, i32),
    max: (i32, i32, i32, i32),
}

impl InfiniteGrid {
    fn set(&mut self, x: i32, y: i32, z: i32, w: i32) {
        self.min = (min(self.min.0, x - 1), min(self.min.1, y - 1), min(self.min.2, z - 1), min(self.min.3, w - 1));
        self.max = (max(self.max.0, x + 1), max(self.max.1, y + 1), max(self.max.2, z + 1), max(self.max.3, w + 1));
        self.grid.insert((x, y, z, w), true);
    }

    fn get(&self, x: i32, y: i32, z: i32, w: i32) -> bool {
        *self.grid.get(&(x, y, z, w)).unwrap_or(&false)
    }

    fn clear(&mut self, x: i32, y: i32, z: i32, w: i32) {
        self.grid.insert((x, y, z, w), false);
    }

    fn new() -> InfiniteGrid {
        InfiniteGrid {
            grid: HashMap::new(),
            min: (0, 0, 0, 0),
            max: (0, 0, 0, 0),
        }
    }

    fn count_active_neighbors(&self, x: i32, y: i32, z: i32, w: i32) -> u32 {
        let mut count = 0;
        for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                for k in z - 1..=z + 1 {
                    for l in w - 1..=w + 1 {
                        if !(x == i && y == j && z == k && w == l) && self.get(i, j, k, l) {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn coordinates(&self) -> InfiniteGridIterator {
        InfiniteGridIterator {
            min: self.min,
            max: self.max,
            current: (self.min.0, self.min.1, self.min.2, self.min.3),
        }
    }
}

struct InfiniteGridIterator {
    min: (i32, i32, i32, i32),
    max: (i32, i32, i32, i32),
    current: (i32, i32, i32, i32),
}

impl Iterator for InfiniteGridIterator {
    type Item = (i32, i32, i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.max {
            return None;
        }
        if self.current.3 < self.max.3 {
            self.current.3 += 1;
        } else {
            if self.current.2 < self.max.2 {
                self.current.2 += 1;
            } else {
                if self.current.1 < self.max.1 {
                    self.current.1 += 1;
                } else {
                    self.current.0 += 1;
                    self.current.1 = self.min.1;
                }
                self.current.2 = self.min.2;
            }
            self.current.3 = self.min.3;
        }
        Some(self.current)
    }
}

fn part_one(mut grid: InfiniteGrid) {
    for _ in 0..6 {
        let mut grid2 = grid.clone();
        for (x, y, z, _) in grid.coordinates() {
            let count = grid.count_active_neighbors(x, y, z, 0);
            let active = grid.get(x, y, z, 0);
            if active && !(count == 2 || count == 3) {
                grid2.clear(x, y, z, 0);
            } else if count == 3 {
                grid2.set(x, y, z, 0);
            }
        }
        grid = grid2;
    }
    println!("{}",
             grid.coordinates()
                 .filter(|(_, _, _, w)| *w == 0)
                 .fold(0, |acc, (x, y, z, _)| acc + if grid.get(x, y, z, 0) { 1 } else { 0 }));
}

fn part_two(mut grid: InfiniteGrid) {
    for _ in 0..6 {
        let mut grid2 = grid.clone();
        for (x, y, z, w) in grid.coordinates() {
            let count = grid.count_active_neighbors(x, y, z, w);
            let active = grid.get(x, y, z, w);
            if active && !(count == 2 || count == 3) {
                grid2.clear(x, y, z, w);
            } else if count == 3 {
                grid2.set(x, y, z, w);
            }
        }
        grid = grid2;
    }
    println!("{}",
             grid.coordinates()
                 .fold(0, |acc, (x, y, z, w)| acc + if grid.get(x, y, z, w) { 1 } else { 0 }));
}

pub fn run(filename: &String) {
    let file = fs::File::open(filename).expect("Unable to open a file");
    let reader = io::BufReader::new(file);

    let mut grid = InfiniteGrid::new();
    let mut x = 0;
    let mut y = 0;
    for line in reader.lines().map(|l| l.unwrap()) {
        for ch in line.chars() {
            match ch {
                '.' => (),
                '#' => grid.set(x, y, 0, 0),
                _   => unreachable!(),
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }
    part_one(grid.clone());
    part_two(grid.clone());
}
