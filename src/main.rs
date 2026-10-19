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

fn insertion_sort(arr: &mut Vec<i32>) -> u64 {
    let mut comparisons: u64 = 0;

    for i in 1..arr.len() {
        let current = arr[i];
        let mut left = 0;
        let mut right = i;

        while left < right {
            comparisons += 1;
            let mid = left + (right - left) / 2;
            if arr[mid] <= current { 
                left = mid + 1; 
            } else { 
                right = mid; 
            }
        }

        arr.insert(left,current);
        arr.remove(i + 1);
    }

    comparisons
}

fn hoara_sort(arr: &mut [i32]) -> u64 {
    fn sort(a: &[i32]) -> (Vec<i32>, u64) {
        if a.len() <= 1 {
            return (a.to_vec(), 0);
        }

        let pivot_index: usize = a.len() as usize / 2;
        let pivot = a[pivot_index];
        let mut left = Vec::new();
        let mut equal = Vec::new();
        let mut right = Vec::new();
        equal.push(pivot);
        let mut comparisons: u64 = 0;
        for (i, value) in a.iter().copied().enumerate() {
            if i == pivot_index as usize { continue; }
            comparisons += 1;

            if value < pivot {
                left.push(value);
            } else {
                if value > pivot {
                    right.push(value);
                } else {
                    equal.push(value);
                }
            }
        }

        let (sorted_left, left_comparisons) = sort(&left);
        let (sorted_right, right_comparisons) = sort(&right);

        comparisons += left_comparisons;
        comparisons += right_comparisons;

        let mut result = Vec::with_capacity(sorted_left.len() + equal.len() + sorted_right.len());
        result.extend(sorted_left);
        result.extend(equal);
        result.extend(sorted_right);
        (result, comparisons)
    }

    let (sorted, comparisons) = sort(arr);
    arr.copy_from_slice(&sorted);
    comparisons
}

fn main() {
    const SIZE: usize = 10_000;

    let mut rng = rand::thread_rng();
    let source: Vec<i32> = (0..SIZE).map(|_| rng.gen_range(-100..100)).collect();

    let mut arr = source.clone();
    let _start = Instant::now();
    let comparisons = bubble_sort(&mut arr);
    println!(
        "Пузырь. Сравнений: {comparisons}"
    );

    let mut arr = source.clone();
    let _start = Instant::now();
    let comparisons = insertion_sort(&mut arr);
    println!(
        "Вставки. Сравнений: {comparisons}"
    );

    let mut arr = source.clone();
    let _start = Instant::now();
    let comparisons = hoara_sort(&mut arr);
    println!(
        "Хоара. Сравнений: {comparisons}"
    );
}
