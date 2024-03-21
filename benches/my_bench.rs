use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn remove_digit(n: i32, digit: i32) -> i32 {
    let n_to_digits: Vec<i32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let mut nums_without_digit = Vec::new();

    for i in 0..n_to_digits.len() {
        let current_digit = n_to_digits[i];
        if current_digit == digit {
            let mut new_num = n_to_digits.clone();
            new_num.remove(i);
            let new_num = new_num.iter().fold(0, |acc, elem| acc * 10 + elem);
            nums_without_digit.push(new_num);
        }
    }

    if nums_without_digit.len() <= 1 {
        return n;
    }

    nums_without_digit.sort();
    nums_without_digit.reverse();
    nums_without_digit[0]
}

pub fn remove_digit_gpt(n: i32, digit: i32) -> i32 {
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

    if max_num == 0 && !n_str.contains(digit_char) {
        return n;
    }

    max_num
}

fn benchmark_remove_digit(c: &mut Criterion) {
    c.bench_function("original_remove_digit", |b| {
        b.iter(|| remove_digit(black_box(31415926), black_box(1)))
    });
    c.bench_function("improved_remove_digit", |b| {
        b.iter(|| remove_digit_gpt(black_box(31415926), black_box(1)))
    });
}

criterion_group!(benches, benchmark_remove_digit);
criterion_main!(benches);
