// iterators4.rs

pub fn factorial_iterative(num: u64) -> u64 {
    let mut r : u64 = 1;
    for i in 1..=num {
        r *= i
    }

    r
}

pub fn factorial_recursive(num: u64) -> u64 {
    match num {
        1 => 1,
        _ => factorial_recursive(num-1) * num
    }
}

pub fn factorial(num: u64) -> u64 {
    //factorial_iterative(num)
    //factorial_recursive(num)
    
    //(1..=num).product()
    //(1u64..=num).fold(1u64, |acc, v| acc * v)
    (1..num + 1).fold(1, |prod, x| prod * x)


    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
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
