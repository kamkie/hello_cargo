use std::ffi::OsString;

fn main() {
    let x = "Hello, world!";
    println!("{}", x);
    let o = OsString::from(x);
    let y: bool = x.eq(&o);
    println!("string is equal to os string: {}", y);

    let vector: Vec<i32> = vec![1, 2, 3];

    if let Some(ble) = vector.get(100) {
        println!("ble: {}", ble);
    } else {
        println!("vector does not have index 100");
    }

    let number = get_or_default(&vector, 1, -1);
    println!("ble2: {}", number);
    let number = get_or_default2(&vector, 100, -1);
    println!("ble3: {}", number);

    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(Some(1).unwrap_or(-1), 1);
    assert_eq!(None.unwrap_or("bike"), "bike");
    assert_eq!(None.unwrap_or(-1), -1);
}

fn get_or_default(ref vec: &Vec<i32>, index: usize, default: i32) -> i32 {
    if let Some(&ble) = vec.get(index) {
        ble
    } else {
        default
    }
}

fn get_or_default2(ref vec: &Vec<i32>, index: usize, default: i32) -> i32 {
    match vec.get(index) {
        Some(&ble) => ble,
        _ => default,
    }
}
