use std::collections::HashMap;

fn gray(n: u8) -> Vec<String> {
    let mut memo = HashMap::new();
    gray_memoized(n, &mut memo)
}

fn gray_memoized(n: u8, memo: &mut HashMap<u8, Vec<String>>) -> Vec<String> {
    if let Some(result) = memo.get(&n) {
        return result.clone();
    }

    let result = if n == 0 {
        vec!["".to_string()]
    } else {
        let lower = gray_memoized(n - 1, memo);
        let mut result = Vec::new();

        for code in &lower {
            result.push(format!("0{}", code));
        }
        for code in lower.iter().rev() {
            result.push(format!("1{}", code));
        }
        result
    };

    memo.insert(n, result.clone());
    result
}

fn main() {
    let test_data = [
        (0, vec!["".to_string()]),
        (1, vec!["0".to_string(), "1".to_string()]),
        (2, vec!["00".to_string(), "01".to_string(), "11".to_string(), "10".to_string()]),
        (3, vec!["000".to_string(), "001".to_string(), "011".to_string(), "010".to_string(),
                 "110".to_string(), "111".to_string(), "101".to_string(), "100".to_string()]),
        (4, vec!["0000".to_string(), "0001".to_string(), "0011".to_string(), "0010".to_string(),
                 "0110".to_string(), "0111".to_string(), "0101".to_string(), "0100".to_string(),
                 "1100".to_string(), "1101".to_string(), "1111".to_string(), "1110".to_string(),
                 "1010".to_string(), "1011".to_string(), "1001".to_string(), "1000".to_string()]),
    ];

    for (n, expected) in test_data.iter() {
        let output = gray(*n);
        println!("Input: {}, Output: {:?}, Expected: {:?}", n, output, expected);
        assert_eq!(output, *expected);
    }

    println!("All manual tests passed!");
}

#[cfg(test)]
mod tests {
    use super::gray;

    #[test]
    fn test() {
        let test_data = [
            (0, vec!["".to_string()]),
            (1, vec!["0".to_string(), "1".to_string()]),
            (2, vec!["00".to_string(), "01".to_string(), "11".to_string(), "10".to_string()]),
            (3, vec!["000".to_string(), "001".to_string(), "011".to_string(), "010".to_string(),
                     "110".to_string(), "111".to_string(), "101".to_string(), "100".to_string()]),
            (4, vec!["0000".to_string(), "0001".to_string(), "0011".to_string(), "0010".to_string(),
                     "0110".to_string(), "0111".to_string(), "0101".to_string(), "0100".to_string(),
                     "1100".to_string(), "1101".to_string(), "1111".to_string(), "1110".to_string(),
                     "1010".to_string(), "1011".to_string(), "1001".to_string(), "1000".to_string()]),
        ];

        for (n, expected) in test_data.iter() {
            let output = gray(*n);
            println!("Input: {}, Output: {:?}, Expected: {:?}", n, output, expected);
            assert_eq!(output, *expected);
        }
    }
}
