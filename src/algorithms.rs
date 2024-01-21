pub fn linear_search<T: PartialEq>(haystack: &[T], needle: T) -> Option<usize> {
    for (i, n) in haystack.iter().enumerate() {
        if *n == needle {
            return Some(i);
        }
    }

    None
}

pub fn binary_search<T: Ord>(haystack: &[T], needle: T) -> Option<usize> {
    let mut lo = 0;
    let mut hi = haystack.len();

    while lo < hi {
        let m = lo + (hi - lo) / 2;
        let v = &haystack[m];

        if *v == needle {
            return Some(m);
        } else if *v > needle {
            hi = m;
        } else {
            lo = m + 1;
        }
    }
    None
}

pub fn bubble_sort<T: Ord + Clone>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                // let tmp = &arr[j].clone();
                // arr[j] = arr[j + 1].clone();
                // arr[j + 1] = tmp.clone();
            }
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

pub fn run() {}
