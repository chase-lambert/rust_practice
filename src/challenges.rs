use std::{
    collections::{HashMap, HashSet},
    format, println,
};

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
pub fn char_to_points(c: char) -> u32 {
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

pub fn scrabble_score(w: &str) -> u32 {
    let w = w.to_uppercase();
    w.chars().fold(0, |acc, c| acc + char_to_points(c))
}

#[test]
fn scrabble_score_test() {
    assert_eq!(scrabble_score("FIZZBUZZ"), 49);
}

// rendezvous with cassidoo challenge: 23.06.25
pub fn missing_letters(letters: &[char]) -> Vec<char> {
    let first = letters[0] as u8;
    let last = letters[letters.len() - 1] as u8;

    let mut pointer = 0;
    let mut missing = Vec::new();

    for i in first..last {
        if i != letters[pointer] as u8 {
            missing.push(i as char);
        } else {
            pointer += 1;
        }
    }

    missing
}

#[test]
fn missing_letters_test() {
    assert_eq!(missing_letters(&['a', 'b', 'c', 'd', 'f']), vec!['e']);

    assert_eq!(
        missing_letters(&[
            'a', 'b', 'c', 'd', 'e', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
            't', 'u', 'w', 'x', 'y', 'z'
        ]),
        vec!['f', 'g', 'v']
    );
}

pub enum Direction {
    Horizontal,
    Vertical,
}

pub fn flip<T>(arr: Vec<Vec<T>>, direction: Direction) -> Vec<Vec<T>> {
    match direction {
        Direction::Horizontal => arr
            .into_iter()
            .map(|row| row.into_iter().rev().collect())
            .collect(),
        Direction::Vertical => arr.into_iter().rev().collect(),
    }
}

#[test]
fn test_flip_horizontal() {
    let array = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let expected = vec![vec![3, 2, 1], vec![6, 5, 4], vec![9, 8, 7]];
    assert_eq!(flip(array, Direction::Horizontal), expected);
}

#[test]
fn test_flip_vertical() {
    let array = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let expected = vec![vec![7, 8, 9], vec![4, 5, 6], vec![1, 2, 3]];
    assert_eq!(flip(array, Direction::Vertical), expected);
}

// rendezvous with cassidoo challenge: 24.02.26
pub fn remove_digit(n: i32, digit: i32) -> i32 {
    let digit_char = char::from_digit(digit as u32, 10).unwrap();
    let n_str = n.to_string();
    let mut max_num = 0;

    for (i, c) in n_str.char_indices() {
        if c == digit_char {
            let new_num_str = [&n_str[..i], &n_str[i + 1..]].concat();
            let new_num = new_num_str.parse::<i32>().unwrap();

            if new_num > max_num {
                max_num = new_num;
            }
        }
    }

    if max_num == 0 {
        return n;
    }

    max_num
}

#[test]
fn remove_digit_test() {
    assert_eq!(3415926, remove_digit(31415926, 1));
    assert_eq!(231, remove_digit(1231, 1));
}

// rendezvous with cassidoo challenge: 24.03.14
pub fn max_gap(nums: &[i32]) -> i32 {
    if nums.len() < 2 {
        return 0;
    }

    let mut nums = nums.to_vec();
    nums.sort();

    nums.windows(2).map(|w| w[1] - w[0]).max().unwrap_or(0)
}

#[test]
fn max_gap_test() {
    let nums = [3, 6, 9, 1, 2];
    assert_eq!(3, max_gap(&nums));
}

// rendezvous with cassidoo challenge: 24.04.01
pub fn unique_sum(nums: &[i32]) -> i32 {
    nums.iter()
        .filter(|&&n| {
            let mut chars_set = std::collections::HashSet::new();
            n.to_string().chars().all(|c| chars_set.insert(c))
        })
        .sum()
}

#[test]
fn unique_sum_test() {
    assert_eq!(6, (unique_sum(&[1, 2, 3])));
    assert_eq!(0, (unique_sum(&[11, 22, 33])));
    assert_eq!(5, (unique_sum(&[101, 2, 3])));
}
