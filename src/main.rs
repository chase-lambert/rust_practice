#![allow(unused)]

mod algorithms;
mod challenges;
mod google_course;

fn main() {}

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
