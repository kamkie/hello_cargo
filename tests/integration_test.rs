extern crate hello_cargo;

#[test]
fn it_starts() {
    hello_cargo::demo()
}


#[test]
fn run_fib() {
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
}