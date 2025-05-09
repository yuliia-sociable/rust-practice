use itertools::Itertools;

fn find_variables() {
    let digits = vec![1, 2, 3, 4, 5, 6, 7, 8]; // Числа від 1 до 8
    let permutations = digits.iter().permutations(8); // Перебір усіх перестановок

    let mut count = 0; // Лічильник рішень

    for perm in permutations {
        let m = perm[0];
        let u = perm[1];
        let x = perm[2];
        let a = perm[3];
        let s = perm[4];
        let l = perm[5];
        let o = perm[6];
        let n = perm[7];

        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;

        if muxa * a == slon {
            count += 1;

            println!(" {}{}{}{}", m, u, x, a);
            println!("x   {}", a);
            println!("------");
            println!(" {}{}{}{}", s, l, o, n);
            println!();
        }
    }

    println!("Знайдено рішень: {}", count);
}

fn main() {
    find_variables();
}
