use rand::Rng;
use std::time::Instant;

fn bubble_sort(arr: &mut [i32]) -> u64 {
    let mut comparisons: u64 = 0;
    let n = arr.len();

    for i in 0..n.saturating_sub(1) {

        for j in 0..n - 1 - i {
            comparisons += 1;

            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }

    }

    comparisons
}

fn insertion_sort(arr: &mut [i32]) -> u64 {
    let mut comparisons: u64 = 0;

    for i in 1..arr.len() {
        let current = arr[i];
        let mut j = i;

        while j > 0 {
            comparisons += 1;

            if arr[j - 1] <= current {
                break;
            }

            arr[j] = arr[j - 1];
            j -= 1;
        }

        arr[j] = current;
    }

    comparisons
}

fn hoara_sort(arr: &mut [i32]) -> u64 {
    fn sort(a: &[i32], comparisons: &mut u64) -> Vec<i32> {
        if a.len() <= 1 {
            return a.to_vec();
        }

        let pivot = a[a.len() / 2];
        let mut left = Vec::new();
        let mut equal = Vec::new();
        let mut right = Vec::new();

        for &value in a {
            *comparisons += 1;

            if value < pivot {
                left.push(value);
            } else if value > pivot {
                *comparisons += 1;
                right.push(value);
            } else {
                *comparisons += 1;
                equal.push(value);
            }
        }

        let sorted_left = sort(&left, comparisons);
        let sorted_right = sort(&right, comparisons);

        let mut result = Vec::with_capacity(sorted_left.len() + equal.len() + sorted_right.len());
        result.extend(sorted_left);
        result.extend(equal);
        result.extend(sorted_right);
        result
    }

    let mut comparisons: u64 = 0;
    let sorted = sort(arr, &mut comparisons);
    arr.copy_from_slice(&sorted);
    comparisons
}

fn is_sorted(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

fn main() {
    const SIZE: usize = 100_000;

    let mut rng = rand::thread_rng();
    let source: Vec<i32> = (0..SIZE).map(|_| rng.gen_range(-100..100)).collect();

    let mut arr = source.clone();
    let start = Instant::now();
    let comparisons = bubble_sort(&mut arr);
    println!(
        "Пузырь. Сравнений: {comparisons}"
    );

    let mut arr = source.clone();
    let start = Instant::now();
    let comparisons = insertion_sort(&mut arr);
    println!(
        "Вставки. Сравнений: {comparisons}"
    );

    let mut arr = source.clone();
    let start = Instant::now();
    let comparisons = hoara_sort(&mut arr);
    println!(
        "Хоара. Сравнений: {comparisons}"
    );
}
