
fn increment_pos(pos: &mut u8, amount: u8) {
    *pos += amount;
    if *pos > 10 {
        *pos = (*pos - 1) % 10 + 1;
    }
}

fn increment_dice(dice: &mut u8) {
    *dice += 1;
    if *dice > 100 {
        *dice = 1;
    }
}

fn simulate_game(start_pos: Vec<u8>) {
    let mut pos = start_pos.clone();
    let mut scores = start_pos.iter().map(|_| 0u32).collect::<Vec<_>>();
    let mut dice_counter = 1;
    let mut num_rolls = 0u32;
    'main: loop {
        for (p, s) in pos.iter_mut().zip(scores.iter_mut()) {
            for _ in 0..3 {
                increment_pos(p, dice_counter);
                increment_dice(&mut dice_counter);
                num_rolls += 1;
            }
            *s += *p as u32;
            if *s >= 1000 {
                break 'main;
            }
        }
    }
    println!("{}", num_rolls * scores.iter().min().unwrap());
}

fn main() {
    let player_1_start = 4;
    let player_2_start = 1;
    simulate_game(vec![player_1_start, player_2_start]);
}
