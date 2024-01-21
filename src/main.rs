#![allow(unused)]

mod algorithms;
mod challenges;
mod google_course;

fn main() {}

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
