fn main() {
    let numbers = [1, 2, 3, 4, 5];
    // let numbers: [i32; 5] = [1, 2, 3, 4, 5]; declare with array type and size

    for n in numbers.iter() {
        println!("{}", n);
    }

    for i in 0..numbers.len() {
        println!("{}", numbers[i]);
    }
}
