// 2. Функція рахує мінімальну кількість переміщень вантажу, щоб на всіх кораблях стало порівну вантажу.
// Ця версія функції не перевіряє, чи можливий рівномірний розподіл.
fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;
    let avg = total / n;

    let mut moves = 0;
    for &cargo in shipments {
        if cargo > avg {
            moves += (cargo - avg) as usize;
        }
    }

    moves
}

// 3. Чи завжди можливо всі кораблі забезпечити однаковою кількістю грузу?
// Ні. Якщо сума вантажу не ділиться на кількість кораблів без остачі — це неможливо.

// 4. Як буде виглядати сигнатура в іншому випадку? Щоб врахувати неможливість розподілу, потрібно змінити тип повернення, наприклад так:  fn count_permutation(shipments: &Vec<u32>) -> isize і тоді повертати -1 у випадку, коли рівномірний розподіл неможливий

// 5. Генерація вектора вантажів, які можуть бути розподілені рівномірно
fn gen_shipments(n: usize) -> Vec<u32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let avg = rng.gen_range(10..100);
    let mut vec = vec![avg; n];

    // Змінюємо деякі значення, зберігаючи суму сталою
    for i in 0..(n / 2) {
        let delta = rng.gen_range(0..=avg.min(10));
        vec[i] += delta;
        vec[n - 1 - i] -= delta;
    }

    vec
}

fn main() {
    let example1 = vec![8, 2, 2, 4, 4];
    let example2 = vec![9, 3, 7, 2, 9];

    println!("Example 1 moves: {}", count_permutation(&example1)); // Очікується: 4
  println!("Example 2 moves: {}", count_permutation(&example2)); // Очікується: 7
   

    let generated = gen_shipments(5);
    println!("Generated shipments: {:?}", generated);
    println!("Moves needed: {}", count_permutation(&generated));
}
