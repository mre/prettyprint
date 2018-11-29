/// Fibonacci implementation in Rust
/// ```
/// assert_eq!(fib(0), 1);
/// assert_eq!(fib(1), 1);
/// assert_eq!(fib(9), 55);
/// ```
pub fn fib(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
