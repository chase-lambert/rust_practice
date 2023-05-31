use std::{collections::HashMap, format, println};

// rendezvous with cassidoo challenge: 23.01.29
pub fn generate_arrays(n: i32) -> Vec<Vec<i32>> {
    (1..=n).map(|x| (1..=x).collect()).collect()
}

#[test]
fn generate_arrays_test() {
    assert_eq!(
        generate_arrays(4),
        vec![vec![1], vec![1, 2], vec![1, 2, 3], vec![1, 2, 3, 4]]
    );

    assert_eq!(generate_arrays(1), vec![vec![1]]);
}

// rendezvous with cassidoo challenge: 23.05.14
pub fn binary_pal(n: i32) -> bool {
    let s = format!("{n:b}");

    s == s.chars().rev().collect::<String>()
}

#[test]
fn binary_pal_test() {
    assert_eq!(binary_pal(5), true);
    assert_eq!(binary_pal(10), false);
}

// rendezvous with cassidoo challenge: 23.05.21
fn char_to_points(c: char) -> u32 {
    match c {
        'E' | 'A' | 'I' | 'O' | 'N' | 'R' | 'T' | 'L' | 'S' | 'U' => 1,
        'D' | 'G' => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 8,
        'Q' | 'Z' => 10,
        _ => 0,
    }
}

fn scrabble_score(w: &str) -> u32 {
    let w = w.to_uppercase();
    w.chars().fold(0, |acc, c| acc + char_to_points(c))
}

#[test]
fn scrabble_score_test() {
    assert_eq!(scrabble_score("FIZZBUZZ"), 49);
}
