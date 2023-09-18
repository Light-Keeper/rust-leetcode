fn memo<A: Copy + PartialEq, B: Copy>(mut f: impl FnMut(A) -> B) -> impl FnMut(A) -> B {
    let mut arg: Option<A> = None;
    let mut res: Option<B> = None;

    return move |x: A| {
        if !arg.is_some_and(|v| v == x) {
            arg = Some(x);
            res = Some(f(x));
        }

        res.unwrap()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let foo = |x| {
            println!("Executing for {}", x);
            x * x
        };

        let mut memoized_foo = memo(foo);

        println!("{}", memoized_foo(1));
        println!("{}", memoized_foo(1));
        println!("{}", memoized_foo(1));
        println!("{}", memoized_foo(2));
        println!("{}", memoized_foo(3));
        println!("{}", memoized_foo(3));
    }
}
