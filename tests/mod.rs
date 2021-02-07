extern crate wifi_rs;

fn fib(n: usize) -> u32 {
    let mut first = 1;
    let mut second = 1;

    for _ in 2..n {
        let temp = first;
        first = second;
        second += temp
    }

    return second;
}

#[test]
fn basic() {
    assert_eq!(fib(10), 55);
}
