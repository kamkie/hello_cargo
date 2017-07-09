use std::ffi::OsString;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::str::FromStr;

extern crate iron;
extern crate time;

use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;

struct ResponseTime;

impl typemap::Key for ResponseTime {
    type Value = u64;
}

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}

fn start_server() {
    let mut chain = Chain::new(hello_world);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    Iron::new(chain).http("localhost:3000").unwrap();
}

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

    let ip: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let ipv6 = Ipv6Addr::from_str("::1").unwrap();
    println!("ip: {}", ip.is_loopback());
    println!("ip2: {}", ip.to_ipv6_mapped().is_loopback());
    println!("ip3: {}", ipv6.is_loopback());

    start_server();
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
