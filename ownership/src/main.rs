fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{s}");

    let s2 = s;
    // s is no longer valid
    println!("{s2}");

    // if you want it to be still valid, you need to do a deep copy:
    let s3 = s2.clone();

    println!("s2: {s2}, s3:{s3}");

    // that's only for objects such as String
    // Other types, such as int, due they are only in stack, they will be still valid
    let a = 5;
    let b = a;

    println!("a: {a}, b: {b}");

    let s4 = String::from("Test");
    without_borrowing(s4);
    // We cant use s4 now, cause the data is deleted!

    // Borrowing
    let mut s4 = String::from("Test");
    with_borrowing(&mut s4); // passing as a reference, by having mut, we can change
    println!("{s4}");

    let s5 = String::from("This is an example test");
    let s6 = first_word(&s5);
    println!("s5: {s5}, s6: {s6}");

    let s7 = String::from("Example");
    let s8 = first_word(&s7);
    println!("s8: {s8}");
}

fn without_borrowing(s: String) {
    // The ownership is now for the s, and when it goes out of scope, the data is going to be
    // deleted
}

fn with_borrowing(s: &mut String) {
    // s doesn't get deleted because it's only a reference. So when function goes out of scope we
    // still are going to have the data
    println!("{s}");
}

fn first_word(s: &String) -> String {
    let mut answer: String = String::from("");
    for char in s.chars() {
        if char == ' ' {
            break;
        }
        answer.push(char);
    }
    answer
}
