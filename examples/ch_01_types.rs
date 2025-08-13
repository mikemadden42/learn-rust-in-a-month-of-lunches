fn main() {
    let first_letter = 'A';
    let space = ' ';
    let my_number = 100;
    let str1 = "hello";

    println!("{first_letter}");
    println!("{space}");
    println!("{my_number}");
    println!("Size of a char: {}", size_of::<char>());
    println!("str1 is {} bytes.", str1.len());
}
