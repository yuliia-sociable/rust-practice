use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| rng.gen_range(10..100)) // Генерація випадкових чисел в діапазоні [10, 99]
        .collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_index = (0, 0);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = (i, i + 1);
        }
    }

    (min_sum, min_index.0, min_index.1)
}

fn print_vector(data: &[i32], min_index1: usize, min_index2: usize) {
    // Виведення індексів
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:2}. ", i);
    }
    println!();

    // Виведення даних
    print!("data:   ");
    for &val in data {
        print!("{:3} ", val);
    }
    println!();

    // Виведення підкреслення для мінімальної пари
    print!("indexes: ");
    for i in 0..data.len() {
        if i == min_index1 {
            print!("\\__ ");
        } else if i == min_index2 {
            print!("__/");
        } else {
            print!("    ");
        }
    }
    println!();
}

fn main() {
    let test_cases = [
        gen_random_vector(20),
        gen_random_vector(20),
        gen_random_vector(20),
        gen_random_vector(20),
    ];

    for data in test_cases.iter() {
        let (min_sum, min_index1, min_index2) = min_adjacent_sum(data);
        print_vector(data, min_index1, min_index2);
        println!("min adjacent sum={}+{}={} at indexes:{},{}", 
                 data[min_index1], 
                 data[min_index2], 
                 min_sum, 
                 min_index1, 
                 min_index2);
        println!();
    }
}
