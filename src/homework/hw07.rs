fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect::<String>()
}

fn main() {
    let test_cases = [
        ("Hello", "hELLO"),
        ("Привіт", "пРИВІТ"),
    ];

    for (input, expected) in test_cases.iter() {
        let output = invert_the_case(input.to_string());
        println!("Input: {}, Output: {}, Expected: {}", input, output, expected);
        assert_eq!(output, expected.to_string());
    }

    println!("All manual tests passed!");
}

#[cfg(test)]
mod tests {
    use super::invert_the_case;

    #[test]
    fn test() {
        let data = [
            ("Hello", "hELLO"),
            ("Привет", "пРИВІТ"),
        ];

        for (a, b) in data.iter() {
            assert_eq!(invert_the_case(a.to_string()), b.to_string());
            assert_eq!(invert_the_case(b.to_string()), a.to_string());
        }
    }
}
