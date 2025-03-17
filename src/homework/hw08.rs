fn is_prime(n: &u32) -> bool {
    if *n <= 1 {
        return false;
    }

    for i in 2..=(f64::from(*n).sqrt() as u32) {
        if *n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let test_cases = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10007, true),
    ];

    for (input, expected) in test_cases.iter() {
        let output = is_prime(input);
        println!("Input: {}, Output: {}, Expected: {}", input, output, expected);
        assert_eq!(output, *expected);
    }

    println!("All manual tests passed!");
}

#[cfg(test)]
mod tests {
    use super::is_prime;

    #[test]
    fn test_is_prime() {
        let test_data = [
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (100, false),
            (10007, true),
        ];

        for (n, prime) in test_data.iter() {
            assert_eq!(is_prime(n), *prime);
        }
    }
}
