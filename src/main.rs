fn print_array(array: &[i8]) {
    // for element in array {
    let length = array.len();
    for (index, element) in (0..length).enumerate() {
        if index < length - 1 {
            print!("{element}, ");
        } else if index == length - 1 {
            println!("{element}");
        }
    }
}

fn main() {
    let int_array: [i8; 10] = [1, 6, 7, 3, 2, 4, 9, 8, 0, 5];
    print_array(&int_array);
}
