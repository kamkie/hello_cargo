extern crate ansi_term;
extern crate iron;
#[macro_use]
extern crate log;
extern crate num;

use iron::prelude::*;
use iron::{AfterMiddleware, BeforeMiddleware, typemap};
use num::bigint::BigUint;
use std::cmp;
use std::ffi::OsString;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::str::FromStr;
use std::time::{Duration, Instant};

struct ResponseTime;

impl typemap::Key for ResponseTime {
    type Value = Instant;
}

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(Instant::now());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = Instant::now().duration_since(*req.extensions.get::<ResponseTime>().unwrap());
        info!("Request took: {delta:?}");
        Ok(res)
    }
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}

pub fn run_fib(n: i32) -> BigUint {
    let start_time = Instant::now();
    let result = fib(n);
    let stop_time = Instant::now();
    let delta: Duration = stop_time.duration_since(start_time);
    let result_dec = decimal_mark3(&result.to_str_radix(10));
    info!("fib: {n} = result: {result_dec} took: {delta:?}");
    result
}

fn decimal_mark3(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + (s.len() / 3));
    if s.len() <= 3 {
        result.push_str(s);
        return result;
    }

    let orfans = s.len() % 3;
    result.push_str(&s[0..orfans]);
    if orfans > 0 {
        result.push(' ');
    }

    for i in (orfans..s.len()).step_by(3) {
        if i >= s.len() {
            break;
        }
        let index_stop = cmp::min(i + 3, s.len());
        result.push_str(&s[i..index_stop]);

        if i + 3 < s.len() {
            result.push(' ');
        }
    }

    result
}

fn fib(n: i32) -> BigUint {
    if n <= 0 {
        return BigUint::from(0u64);
    }
    if n == 1 || n == 2 {
        return BigUint::from(1u64);
    }
    let mut fib_prev: BigUint = BigUint::from(1u64);
    let mut fib_current: BigUint = BigUint::from(2u64);
    for _ in 3..n {
        let sum = fib_prev + &fib_current;
        fib_prev = fib_current;
        fib_current = sum;
    }
    fib_current
}

//fn fib(n: i32) -> u128 {
//    if n <= 0 {
//        return 0;
//    }
//    if n == 1 {
//        return 1;
//    }
//    let mut fib_prev: u128 = 0;
//    let mut fib_current: u128 = 1;
//    for i in 0..(n - 1) {
//        let (sum, overflow) = fib_prev.overflowing_add(fib_current);
//        if overflow {
//            warn!("overflow reached for fib({})", i)
//        }
//        fib_prev = fib_current;
//        fib_current = sum;
//    }
//    fib_current
//}

//fn fib(n: i32) -> i64 {
//    fib(n - 2) + fib(n - 1)
//}

//fn fib(n: i32) -> i64 {
//    match n {
//        0 => 0,
//        n => fibonacci_tail(n, 1, 0, 1),
//    }
//}

//fn fibonacci_tail(n: i32, m: i32, fib_prev: i64, fib_current: i64) -> i64 {
//    if n == m {
//        return fib_current;
//    } else {
//        return fibonacci_tail(n, m + 1, fib_current, fib_prev + fib_current);
//    }
//}

//fn fib(n: i32) -> i64 {
//    match n {
//        0 => 0,
//        1 => 1,
//        2 => 1,
//        3 => 2,
//        n => fib(n - 2) + fib(n - 1),
//    }
//}

pub fn start_server() {
    let mut chain = Chain::new(hello_world);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    Iron::new(chain).http("127.0.0.1:3000").unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works0() {
        assert_eq!(Some("car").unwrap_or("bike"), "car");
        assert_eq!(Some(1).unwrap_or(-1), 1);
        assert_eq!(None.unwrap_or("bike"), "bike");
        assert_eq!(None.unwrap_or(-1), -1);
    }

    #[test]
    #[should_panic(expected = "Make this test fail")]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn decimal_mark3_test() {
        use crate::decimal_mark3;
        assert_eq!(decimal_mark3(&String::from("")), "");
        assert_eq!(decimal_mark3(&String::from("1")), "1");
        assert_eq!(decimal_mark3(&String::from("12")), "12");
        assert_eq!(decimal_mark3(&String::from("123")), "123");
        assert_eq!(decimal_mark3(&String::from("123456")), "123 456");
        assert_eq!(decimal_mark3(&String::from("1234567")), "1 234 567");
        assert_eq!(decimal_mark3(&String::from("12345678")), "12 345 678");
    }
}

pub fn demo() {
    let x = "Hello, world!";
    info!("{}", x);
    let o = OsString::from(x);
    let y: bool = x.eq(&o);
    info!("string is equal to os string: {}", y);

    let vector: Vec<i32> = vec![1, 2, 3];

    if let Some(number) = vector.get(100) {
        info!("ble1: {}", number);
    } else {
        info!("vector does not have index 100");
    }

    let number = get_or_default(&vector, 1, -1);
    info!("ble2: {}", number);
    let number = get_or_default2(&vector, 100, -1);
    info!("ble3: {}", number);
    let number = *vector.get(100).unwrap_or(&-1);
    info!("ble4: {}", number);

    do_something([1, 2, 3, 4]);

    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(Some(1).unwrap_or(-1), 1);
    assert_eq!(None.unwrap_or("bike"), "bike");
    assert_eq!(None.unwrap_or(-1), -1);

    let ip: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let ipv6 = Ipv6Addr::from_str("::1").unwrap();
    info!("ip: {}", ip.is_loopback());
    info!("ip2: {}", ip.to_ipv6_mapped().is_loopback());
    info!("mip3: {}", ipv6.is_loopback());

    use ansi_term::Colour::{Blue, Yellow};
    info!(
        "Demonstrating {} and {}!",
        Blue.bold().paint("blue bold"),
        Yellow.underline().paint("yellow underline")
    );
    info!("Yellow on blue: {}", Yellow.on(Blue).paint("wow!"));
}

fn get_or_default(vec: &[i32], index: usize, default: i32) -> i32 {
    if let Some(&ble) = vec.get(index) {
        ble
    } else {
        default
    }
}

fn get_or_default2(vec: &[i32], index: usize, default: i32) -> i32 {
    match vec.get(index) {
        Some(&ble) => ble,
        _ => default,
    }
}

fn do_something(args: [u8; 4]) {
    info!("args: {:?}", args);
}
