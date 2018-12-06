fn main() {
    let v = vec![1, 2, 3, 4 ,5];
    let v_index = 2;

    match v.get(v_index) {
        Some(_) => { println!("Reachable element at index: {}", v_index); },
        None => { println!("Unreachable element at index: {}", v_index); }
    }
}
