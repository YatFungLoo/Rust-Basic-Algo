#![allow(dead_code)]
#![allow(unused_variables)]

use std::cmp;
use std::cmp::Ordering;
use std::env;

mod binary_heap;
mod queue;

pub fn print_array(array: &[i8]) {
    // for element in array
    let length = array.len() - 1;
    for (index, element) in array.iter().enumerate() {
        match index.cmp(&length) {
            Ordering::Less => {
                print!("{element}, ")
            }
            Ordering::Equal => {
                println!("{element}");
            }
            Ordering::Greater => {}
        }
    }
}

pub fn selection_sort(array: &mut [i8]) {
    let length = array.len();
    for i in 0..length {
        for j in i + 1..length {
            if array[j] < array[i] {
                array.swap(i, j);
            }
        }
    }
}

pub fn insertion_sort(array: &mut [i8]) {
    let length = array.len();
    for i in 1..=length {
        for j in (1..i).rev() {
            if array[j] < array[j - 1] {
                array.swap(j, j - 1);
            }
        }
    }
}

pub fn shell_sort(array: &mut [i8]) {
    let length = array.len();
    let mut h_gap = 1;

    // TODO: learn why choose these value?
    while h_gap < length / 3 {
        h_gap = 3 * h_gap + 1; // 1, 4, 13, 40, 121, ...
    }

    while h_gap >= 1 {
        for i in h_gap..=length {
            for j in (h_gap..i).step_by(h_gap).rev() {
                if array[j] < array[j - h_gap] {
                    array.swap(j, j - h_gap);
                }
            }
        }
        h_gap /= 3;
    }
}

// lo is the start index of the first half, mid index is where the each sorted seciton divides, hi is the end index of the second half.
pub fn abs_inplace_merge_sort(array: &mut [i8], lo: usize, mid: usize, hi: usize) {
    let mut aux: Vec<i8> = Vec::new();
    for index in lo..=hi {
        aux.push(array[index]);
    }

    let mut i = lo;
    let mut j = mid + 1;
    for k in lo..=hi {
        if i > mid {
            // left array is exhausted.
            array[k] = aux[j];
            j = j + 1;
        } else if j > hi {
            // right array is exhausted.
            array[k] = aux[i];
            i = i + 1;
        } else if aux[j] < aux[i] {
            array[k] = aux[j];
            j = j + 1;
        } else {
            array[k] = aux[i];
            i = i + 1;
        }
    }
}

// Reuse vector everytime due to vector no having the correct index for right side sorts.
pub fn merge_sort(array: &mut [i8], aux: &mut [i8], lo: usize, mid: usize, hi: usize) {
    for index in lo..=hi {
        aux[index] = array[index];
    }

    let mut i = lo;
    let mut j = mid + 1;
    for k in lo..=hi {
        if i > mid {
            // left array is exhausted.
            array[k] = aux[j];
            j += 1;
        } else if j > hi {
            // right array is exhausted.
            array[k] = aux[i];
            i += 1;
        } else if aux[j] < aux[i] {
            array[k] = aux[j];
            j += 1;
        } else {
            array[k] = aux[i];
            i += 1;
        }
    }
}

// uses abs_inplace_merge_sort().
pub fn topdown_merge_sort(array: &mut [i8], aux: &mut [i8], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }

    let mid: usize = lo + (hi - lo) / 2;

    topdown_merge_sort(array, aux, lo, mid);
    topdown_merge_sort(array, aux, mid + 1, hi);

    merge_sort(array, aux, lo, mid, hi);
}

// uses abs_inplace_merge_sort().
pub fn bottomup_merge_sort(array: &mut [i8], aux_array: &mut [i8]) {
    let length = array.len();
    let mut sz: usize = 1;
    let mut lo: usize = 0;

    while sz < length {
        while lo < (length - sz) {
            merge_sort(
                array,
                aux_array,
                lo,
                lo + sz - 1,
                cmp::min(lo + sz + sz - 1, length - 1),
            );
            lo += sz + sz;
        }
        lo = 0; // Reset index.
        sz += sz;
    }
}

pub fn quick_sort(array: &mut [i8], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    let j = quick_sort_partition(array, lo, hi);
    if j == 0 {
        quick_sort(array, lo, j);
    } else {
        quick_sort(array, lo, j - 1);
    }
    quick_sort(array, j + 1, hi);
}

pub fn quick_sort_partition(array: &mut [i8], lo: usize, hi: usize) -> usize {
    let mut i = lo + 1;
    let mut j = hi;
    let c = array[lo]; // int to be 'c'ompared.

    loop {
        while array[i] < c {
            if i == hi {
                break;
            }
            i += 1;
        }
        while c < array[j] {
            if j == lo {
                break;
            }
            j -= 1;
        }
        if i >= j {
            break;
        }
        array.swap(i, j);
    }
    array.swap(lo, j);
    j
}

pub fn quick_sort_3way(array: &mut [i8], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    let mut lt = lo;
    let mut i = lo + 1;
    let mut gt = hi;
    let c = array[lo]; // int to be 'c'ompared.

    while i <= gt {
        match array[i].cmp(&c) {
            Ordering::Less => {
                array.swap(lt, i);
                lt += 1;
                i += 1;
            }
            Ordering::Equal => {
                i += 1;
            }
            Ordering::Greater => {
                array.swap(i, gt);
                gt -= 1;
            }
        }
    }
    if lt == 0 {
        quick_sort(array, lo, lt);
    } else {
        quick_sort(array, lo, lt - 1);
    }
    quick_sort(array, gt + 1, hi);
}

pub fn binary_search(array: &[i8], lo: usize, hi: usize, key: i8) -> usize {
    if hi < lo {
        return lo;
    }
    let mid = lo + ((lo + hi) / 2);
    match array[mid].cmp(&key) {
        Ordering::Less => {
            binary_search(array, mid + 1, hi, key)
        }
        Ordering::Equal => mid,
        Ordering::Greater => {
            binary_search(array, lo, mid, key)
        }
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ntest::timeout;

    #[test]
    fn selection_sort_test() {
        let mut int_array: [i8; 10] = [1, 6, 7, 3, 2, 4, 9, 8, 0, 5];
        let answer: [i8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        selection_sort(&mut int_array);
        assert_eq!(int_array, answer);
    }

    #[test]
    fn insertion_sort_test() {
        let mut int_array: [i8; 10] = [1, 6, 7, 3, 2, 4, 9, 8, 0, 5];
        let answer: [i8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        insertion_sort(&mut int_array);
        assert_eq!(int_array, answer);
    }

    #[test]
    fn shell_sort_test() {
        let mut int_array: [i8; 40] = [
            1, 6, 7, 3, 2, 4, 9, 8, 0, 5, 1, 6, 7, 3, 2, 4, 9, 8, 0, 5, 1, 6, 7, 3, 2, 4, 9, 8, 0,
            5, 1, 6, 7, 3, 2, 4, 9, 8, 0, 5,
        ];
        let answer: [i8; 40] = [
            0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7,
            7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9,
        ];
        shell_sort(&mut int_array);
        assert_eq!(int_array, answer);
    }

    #[test]
    fn abs_inplace_merge_sort_test() {
        let mut int_array: [i8; 10] = [1, 2, 3, 6, 7, 0, 4, 5, 8, 9];
        let answer: [i8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let lo: usize = 0;
        let hi: usize = int_array.len() - 1;
        let mid: usize = 4; // One index before the start of the second sorted half.
        abs_inplace_merge_sort(&mut int_array, lo, mid, hi);
        assert_eq!(int_array, answer);
    }

    #[test]
    fn topdown_merge_sort_test() {
        let mut int_array: [i8; 15] = [6, 1, 3, 7, 2, 4, 9, 8, 0, 5, 3, 5, 0, 9, 8];
        let mut aux_array: [i8; 15] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let answer: [i8; 15] = [0, 0, 1, 2, 3, 3, 4, 5, 5, 6, 7, 8, 8, 9, 9];
        let lo: usize = 0;
        let hi: usize = int_array.len() - 1;
        topdown_merge_sort(&mut int_array, &mut aux_array, lo, hi);
        assert_eq!(int_array, answer);
    }

    #[test]
    fn bottomup_merge_sort_test() {
        let mut int_array: [i8; 15] = [6, 1, 3, 7, 2, 4, 9, 8, 0, 5, 3, 5, 0, 9, 8];
        let mut aux_array: [i8; 15] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let answer: [i8; 15] = [0, 0, 1, 2, 3, 3, 4, 5, 5, 6, 7, 8, 8, 9, 9];
        bottomup_merge_sort(&mut int_array, &mut aux_array);
        assert_eq!(int_array, answer);
    }

    #[test]
    fn quick_sort_test() {
        let mut int_array: [i8; 15] = [6, 1, 3, 7, 2, 4, 9, 8, 0, 5, 3, 5, 0, 9, 8];
        let answer: [i8; 15] = [0, 0, 1, 2, 3, 3, 4, 5, 5, 6, 7, 8, 8, 9, 9];
        let length = int_array.len() - 1;
        quick_sort(&mut int_array, 0, length);
        assert_eq!(int_array, answer);
    }

    #[test]
    #[timeout(100)]
    fn quick_sort_test_long_array() {
        let mut int_array: [i8; 40] = [
            1, 6, 7, 3, 2, 4, 9, 8, 0, 5, 1, 6, 7, 3, 2, 4, 9, 8, 0, 5, 1, 6, 7, 3, 2, 4, 9, 8, 0,
            5, 1, 6, 7, 3, 2, 4, 9, 8, 0, 5,
        ];
        let answer: [i8; 40] = [
            0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7,
            7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9,
        ];
        let length = int_array.len() - 1;
        quick_sort(&mut int_array, 0, length);
        assert_eq!(int_array, answer);
    }

    #[test]
    fn quick_sort_3way_test() {
        let mut int_array: [i8; 15] = [6, 1, 3, 7, 2, 4, 9, 8, 0, 5, 3, 5, 0, 9, 8];
        let answer: [i8; 15] = [0, 0, 1, 2, 3, 3, 4, 5, 5, 6, 7, 8, 8, 9, 9];
        let length = int_array.len() - 1;
        quick_sort_3way(&mut int_array, 0, length);
        assert_eq!(int_array, answer)
    }

    #[test]
    #[timeout(100)]
    fn quick_sort_3way_test_long_array() {
        let mut int_array: [i8; 40] = [
            1, 6, 7, 3, 2, 4, 9, 8, 0, 5, 1, 6, 7, 3, 2, 4, 9, 8, 0, 5, 1, 6, 7, 3, 2, 4, 9, 8, 0,
            5, 1, 6, 7, 3, 2, 4, 9, 8, 0, 5,
        ];
        let answer: [i8; 40] = [
            0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7,
            7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9,
        ];
        let length = int_array.len() - 1;
        quick_sort_3way(&mut int_array, 0, length);
        assert_eq!(int_array, answer)
    }
}
