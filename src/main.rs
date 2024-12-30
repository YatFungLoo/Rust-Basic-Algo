fn print_array(array: &[i8]) {
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

fn selection_sort(array: &mut [i8]) {
    let length = array.len();
    for i in 0..length {
	for j in i+1..length {
	    if array[j] < array[i] {
		array.swap(i, j);
	    }
	}
    }
}

fn main() {
    let mut int_array: [i8; 10] = [1, 6, 7, 3, 2, 4, 9, 8, 0, 5];
    selection_sort(&mut int_array);
    print_array(&int_array);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	let mut int_array: [i8; 10] = [1, 6, 7, 3, 2, 4, 9, 8, 0, 5];
	let answer: [i8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        selection_sort(&mut int_array);
        assert_eq!(int_array, answer);
    }
}
