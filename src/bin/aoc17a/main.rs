use ndarray::prelude::*;
use std::cmp::max;

struct Target {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

fn simulate_trajectory(start_vel: Array1<i32>, target: &Target) -> Option<i32> {
    let mut pos = arr1(&[0, 0]);
    let mut vel = start_vel.clone();
    let mut max_y = 0;
    loop {
        pos += &vel;
        if vel[0] > 0 {
            vel[0] -= 1;
        }
        vel[1] -= 1;
        max_y = max(max_y, pos[1]);
        if target.x_min <= pos[0] && pos[0] <= target.x_max && target.y_min <= pos[1] && pos[1] <= target.y_max {
            return Some(max_y);
        }
        if pos[0] > target.x_max || pos[1] < target.y_min && vel[1] <= 0 {
            return None;
        }
    }
}

fn find_max_y(target: Target) -> i32 {
    let mut max_y = 0;
    for vel_y in target.y_min..target.x_max {
        for vel_x in 1..target.x_max {
            max_y = max(
                max_y,
                simulate_trajectory(arr1(&[vel_x, vel_y]), &target).unwrap_or(0),
            );
        }
    }
    max_y
}

fn main() {
    println!(
        "{}",
        find_max_y(Target {
            x_min: 20,
            x_max: 30,
            y_min: -10,
            y_max: -5
        })
    );
    println!(
        "{}",
        find_max_y(Target {
            x_min: 206,
            x_max: 250,
            y_min: -105,
            y_max: -57
        })
    );
}
