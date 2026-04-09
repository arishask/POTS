fn fib(n: u32) -> u32 {
    if n < 2 {
    // первый шаг.
        return n;
    } else {
    // рекурсия.
        let mut f1 = 0;
        let mut f2 = 1;
        let mut f = 0;
        for _ in 1..n {
            f = f1 + f2;
            f2 = f1;
            f1 = f;
        }
        return f;
    }
}

fn main() {
    let n = 15;
    println!("fib({n}) = {}", fib(n));
}