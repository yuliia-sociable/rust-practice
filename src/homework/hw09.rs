fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }

    // Обчислюємо реальний зсув (позитивний або негативний)
    let n = ((n % len as isize) + len as isize) % len as isize; // Це забезпечує правильний зсув навіть при негативних значеннях

    let n = n as usize; // Конвертуємо назад в usize для використання в індексації
    let (left, right) = s.split_at(len - n); // Розділяємо рядок на дві частини
    format!("{}{}", right, left) // З’єднуємо частини
}

fn main() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    for (input, expected) in shifts.iter() {
        let output = rotate(s.clone(), *input);
        println!("Input: {}, Output: {}, Expected: {}", input, output, expected);
        assert_eq!(output, expected.to_string());
    }

    println!("All manual tests passed!");
}

#[cfg(test)]
mod tests {
    use super::rotate;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        for (n, exp) in shifts.iter() {
            let output = rotate(s.to_string(), *n);
            println!("Input: {}, Output: {}, Expected: {}", n, output, exp);
            assert_eq!(output, exp.to_string());
        }
    }
}
