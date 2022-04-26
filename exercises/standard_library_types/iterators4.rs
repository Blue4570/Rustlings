// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    //Using std::iter:Iterator::fold
    //Created a range slice using (1..=num)
    //fold takes an initial value for v, and a closure with type parameters
    //The first param is an accumulator which I have named acc, and 1 gets passed to value
    //on the first iteration
    //Refer to the std lib documentation for more details: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
    //(1..=num).fold(1, |acc, v| acc * v)

    //Another option would be to use reduce: std::iter:Iterator::reduce
    //Reduce is the similar to fold(), but takes the first element as the initial value
    //Refer to the std lib documentation for more details: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.reduce
    (1..=num).reduce(|sum, v| sum * v).unwrap()

    //Or you could use recursion as the traditional approach
    //Obviously using functional programming style requires less code
    //Rust compiler also provides optimizations when using the functional style
    //that are proven to perform slightly better than using the traditional looping or recursion technique

    // if num <= 1 {
    //     num
    // } else {
    //     num * factorial(num -1)
    // }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
