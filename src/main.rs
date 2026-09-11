use rand::Rng;
use std::thread;

fn bubble_sort(arr: &mut [i32]) -> u64 {
    let mut comparisons: u64 = 0;
    let n = arr.len();

    for i in 0..n.saturating_sub(1) {
        let mut swapped = false;

        for j in 0..n - 1 - i {
            comparisons += 1;

            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }

    comparisons
}

fn insertion_sort(arr: &mut [i32]) -> u64 {
    let mut comparisons: u64 = 0;
    let n = arr.len();

    for i in 1..n {
        let current = arr[i];
        let mut j = i as isize - 1;

        while j >= 0 {
            comparisons += 1;

            if arr[j as usize] <= current {
                break;
            }

            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }

        arr[(j + 1) as usize] = current;
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
            } else {
                *comparisons += 1;

                if value > pivot {
                    right.push(value);
                } else {
                    equal.push(value);
                }
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

fn main() {
    const SIZE: usize = 100_000;

    let mut rng = rand::thread_rng();
    let source: Vec<i32> = (0..SIZE).map(|_| rng.gen_range(-100..100)).collect();

    let bubble_array = source.clone();
    let insertion_array = source.clone();
    let hoara_array = source.clone();

    let bubble_thread = thread::spawn(move || {
        let mut arr = bubble_array;
        let comparisons = bubble_sort(&mut arr);
        println!("Пузырь. Сравнений: {comparisons}");
    });

    let insertion_thread = thread::spawn(move || {
        let mut arr = insertion_array;
        let comparisons = insertion_sort(&mut arr);
        println!("Вставки. Сравнений: {comparisons}");
    });

    let hoara_thread = thread::spawn(move || {
        let mut arr = hoara_array;
        let comparisons = hoara_sort(&mut arr);
        println!("Хоара. Сравнений: {comparisons}");
    });

    bubble_thread.join().unwrap();
    insertion_thread.join().unwrap();
    hoara_thread.join().unwrap();
}
