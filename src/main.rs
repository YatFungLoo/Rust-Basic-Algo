pub fn print_array(array: &[i8]) {
    // for element in array {
    let length = array.len();
    for (index, element) in array.iter().enumerate() {
        if index < length - 1 {
            print!("{element}, ");
        } else if index == length - 1 {
            println!("{element}");
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
            for j in (1..i).step_by(h_gap).rev() {
                if array[j] < array[j - 1] {
                    array.swap(j, j - 1);
                }
            }
        }
        h_gap = h_gap / 3;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut int_array: [i8; 10] = [1, 6, 7, 3, 2, 4, 9, 8, 0, 5];
        let answer: [i8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        shell_sort(&mut int_array);
        assert_eq!(int_array, answer);
    }
}
