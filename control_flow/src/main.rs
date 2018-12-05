fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("looping {}", counter);
        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
