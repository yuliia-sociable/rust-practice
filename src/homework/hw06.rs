fn draw_tree(triangles: usize) {
    // Додаємо нульовий рівень через ітератор
    (0..1).for_each(|_| {
        let padding = " ".repeat(triangles * 2);
        println!("{}*", padding);
    });
    
    (0..triangles).for_each(|t| {
        (0..(t + 2)).for_each(|i| {
            let stars = (0..(2 * i + 1)).map(|_| "*").collect::<String>();
            let padding = " ".repeat(triangles * 2 - i);
            println!("{}{}", padding, stars);
        });
    });
}

fn main() {
    let triangles = 5; // Кількість трикутників
    draw_tree(triangles);
}
