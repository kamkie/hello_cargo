#[macro_use]
extern crate log;
extern crate simple_logger;

extern crate hello_cargo;

fn main() {
    simple_logger::init_with_level(log::LogLevel::Debug).unwrap();
    info!("starting app");

    hello_cargo::run_fib(1);
    hello_cargo::run_fib(2);
    hello_cargo::run_fib(3);
    hello_cargo::run_fib(4);
    hello_cargo::run_fib(5);
    hello_cargo::run_fib(6);
    hello_cargo::run_fib(7);
    hello_cargo::run_fib(8);
    hello_cargo::run_fib(9);
    hello_cargo::run_fib(10);
    hello_cargo::run_fib(50);
    hello_cargo::run_fib(150);
    hello_cargo::run_fib(184);
    hello_cargo::run_fib(500);
    //    hello_cargo::run_fib(1_000);
    //    hello_cargo::run_fib(10_000);
    //    hello_cargo::run_fib(100_000);
    //    hello_cargo::run_fib(1_000_000);
    //    hello_cargo::demo();

    hello_cargo::start_server();
}
