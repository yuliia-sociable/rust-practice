// Функція для обчислення найбільшого спільного дільника (GCD) за допомогою алгоритму Евкліда
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let num1 = 24;
    let num2 = 16;

    println!("GCD of {} and {} is {}", num1, num2, gcd(num1, num2));
}
