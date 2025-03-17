fn is_palindrome(x: u32) -> bool {
    let original = x;
    let mut reversed = 0;
    let mut num = x;

    // Реверсуємо число
    while num > 0 {
        reversed = reversed * 10 + num % 10;
        num /= 10;
    }

    original == reversed
}

fn main() {
    let data = [
        (123, false),
        (121, true),
        (1221, true),
    ];

    for (n, exp) in data.iter() {
        let output = is_palindrome(*n);
        println!("Input: {}, Output: {}, Expected: {}", n, output, exp);
        assert_eq!(output, *exp);
    }

    println!("All manual tests passed!");
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn test() {
        let data = [
            (123, false),
            (121, true),
            (1221, true),
        ];

        data.iter().for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
    }
}
