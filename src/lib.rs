pub const NUMBER: u64 = 42;

pub fn fib(number: u64) -> u64 {
    if number == 0 {
        return 0;
    } else if number == 1 {
        return 1;
    }
    fib(number - 1) + fib(number - 2)
}
