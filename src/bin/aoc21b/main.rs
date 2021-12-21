use std::cmp::max;
use std::collections::HashMap;

fn increment_pos(pos: u8, amount: u8) -> u8 {
    let mut pos_new = pos;
    pos_new += amount;
    if pos_new > 10 {
        pos_new = (pos_new - 1) % 10 + 1;
    }
    pos_new
}

fn simulate_roll(pos: u8, count: u64) -> HashMap<u8, u64> {
    let mut player_pos_counts = HashMap::from([(pos, count)]);
    for _ in 0..3 {
        let mut player_pos_counts_new = HashMap::new();
        for dice_roll in 1..=3 {
            for (&p_, &c_) in player_pos_counts.iter() {
                let p_new = increment_pos(p_, dice_roll);
                *player_pos_counts_new.entry(p_new).or_insert(0) += c_;
            }
        }
        player_pos_counts = player_pos_counts_new;
    }
    player_pos_counts
}

fn update_score(pos_counts: &HashMap<u8, u64>, score: u8) -> HashMap<(u8, u8), u64> {
    let mut pos_score_counts = HashMap::new();
    for (&p_, &c_) in pos_counts.iter() {
        *pos_score_counts.entry((p_, score + p_)).or_insert(0) += c_;
    }
    pos_score_counts
}

fn remove_wins(pos_score_counts: &mut HashMap<(u8, u8), u64>) -> u64 {
    let mut count = 0;
    let mut pos_score_counts_new = pos_score_counts.clone();
    for (&(p, s), &c) in pos_score_counts.iter() {
        if s >= 21 {
            count += c;
            pos_score_counts_new.remove(&(p, s));
        }
    }
    *pos_score_counts = pos_score_counts_new;
    count
}

fn simulate_game(start_pos: (u8, u8)) {
    let mut score_pos_counts = HashMap::from([((start_pos, (0u8, 0u8)), 1u64)]);
    let mut num_p1_wins = 0;
    let mut num_p2_wins = 0;
    while score_pos_counts.len() > 0 {
        let mut score_pos_counts_new = HashMap::new();
        for (&((p1, p2), (s1, s2)), &c) in score_pos_counts.iter() {
            if c == 0 {
                continue;
            }
            let player1_pos_counts = simulate_roll(p1, c);
            let mut player1_pos_score_counts = update_score(&player1_pos_counts, s1);
            num_p1_wins += remove_wins(&mut player1_pos_score_counts);
            for (&(p1_new, s1_new), &c1_new) in player1_pos_score_counts.iter() {
                let player2_pos_counts = simulate_roll(p2, c1_new);
                let mut player2_pos_score_counts = update_score(&player2_pos_counts, s2);
                num_p2_wins += remove_wins(&mut player2_pos_score_counts);
                for (&(p2_new, s2_new), &c_new) in player2_pos_score_counts.iter() {
                    *score_pos_counts_new.entry(((p1_new, p2_new), (s1_new, s2_new))).or_insert(0) += c_new;
                }
            }
        }
        score_pos_counts = score_pos_counts_new;
        println!("{} {} {} {}", score_pos_counts.len(), score_pos_counts.values().sum::<u64>(), num_p1_wins, num_p2_wins);
    }
    println!("{}", max(num_p1_wins, num_p2_wins));
}

fn main() {
    let player_1_start = 4;
    let player_2_start = 1;
    simulate_game((player_1_start, player_2_start));
}
