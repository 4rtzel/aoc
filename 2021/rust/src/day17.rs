use std::fs::File;
use std::io;
use std::io::BufRead;

fn calculate_nth_position(x_velocity: i32, y_velocity: i32, mut step: i32) -> (i32, i32) {
    let y = y_velocity * step - (step - 1) * step / 2;
    if x_velocity - step < 0 {
        step = x_velocity;
    }
    let x = x_velocity * step - (step - 1) * step / 2;
    (x , y)
}

fn calculate_next(state: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
    let x = state.0;
    let y = state.1;
    let x_velocity = state.2;
    let y_velocity = state.3;
    (x + x_velocity, y + y_velocity, if x_velocity < 0 { x_velocity + 1 } else if x_velocity > 0 { x_velocity - 1 } else { 0 }, y_velocity -1 )
}

fn will_visit_the_area(x_velocity: i32, y_velocity: i32, area: (i32, i32, i32, i32)) -> (bool, i32) {
    let mut state = (0, 0, x_velocity, y_velocity);
    let mut max_height = 0;
    loop {
        max_height = std::cmp::max(max_height, state.1);
        if state.0 >= area.0 && state.0 <= area.1 && state.1 >= area.2 && state.1 <= area.3 {
            return (true, max_height);
        }
        if state.1 < area.2 {
            return (false, max_height);
        }
        state = calculate_next(state);
    }
}

pub fn run(reader: io::BufReader<File>) {
    let area: Vec<i32> = reader
        .lines()
        .map(|l| l.unwrap())
        .nth(0)
        .unwrap()
        .split(|c: char| !(c.is_numeric() || c == '-'))
        .filter(|c| !c.is_empty())
        .map(|c| c.parse().unwrap())
        .collect();
    let area: (i32, i32, i32, i32) = (area[0], area[1], area[2], area[3]);
    let mut x_velocity = 1;
    while !(area.0..=area.1).contains(&calculate_nth_position(x_velocity, 1, 999).0) {
        x_velocity += 1;
    }
    let mut max_height = 0;
    for y_velocity in 1..999 {
        let res = will_visit_the_area(x_velocity, y_velocity, area);
        if res.0 {
            max_height = std::cmp::max(max_height, res.1);
        }
    }
    println!("{}", max_height);
    let mut number_of_velocities = 0;
    for x in 1..=area.1 {
        for y in std::cmp::min(area.2, area.3)..=-std::cmp::min(area.2, area.3) {
            if will_visit_the_area(x, y, area).0 {
                number_of_velocities += 1;
            }
        }
    }
    println!("{}", number_of_velocities);
}
