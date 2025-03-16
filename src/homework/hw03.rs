const WIDTH: usize = 30;  
const HEIGHT: usize = 15;  

fn main() {  
    let mut lines = vec![String::new(); HEIGHT];  

    for y in 0..HEIGHT {  
        for x in 0..WIDTH {  
            let c = if y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1 {  
                '*' // Малюємо рамку  
            } else if x == y * 2 || x == WIDTH - 1 - y * 2 {  
                '*' // Малюємо діагональні лінії  
            } else {  
                ' ' // Заповнюємо пробілами  
            };  
            lines[y].push(c);  
        }  
    }  

    print!("{}", lines.join("\n"));  
}
