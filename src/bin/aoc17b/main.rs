use ndarray::prelude::*;
use std::cmp::max;

struct Target {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

fn simulate_trajectory(start_vel: Array1<i32>, target: &Target) -> bool {
    let mut pos = arr1(&[0, 0]);
    let mut vel = start_vel.clone();
    loop {
        pos += &vel;
        if vel[0] > 0 {
            vel[0] -= 1;
        }
        vel[1] -= 1;
        if target.x_min <= pos[0] && pos[0] <= target.x_max && target.y_min <= pos[1] && pos[1] <= target.y_max {
            return true;
        }
        if pos[0] > target.x_max || pos[1] < target.y_min && vel[1] <= 0 {
            return false;
        }
    }
}

fn find_num_options(target: Target) -> u32 {
    let mut num_options = 0;
    for vel_y in target.y_min..=2*target.x_max {
        for vel_x in 1..=target.x_max {
            if simulate_trajectory(arr1(&[vel_x, vel_y]), &target) {
                num_options += 1;
            }
        }
    }
    num_options
}

fn main() {
    println!(
        "{}",
        find_num_options(Target {
            x_min: 20,
            x_max: 30,
            y_min: -10,
            y_max: -5
        })
    );
    println!(
        "{}",
        find_num_options(Target {
            x_min: 206,
            x_max: 250,
            y_min: -105,
            y_max: -57
        })
    );
}
