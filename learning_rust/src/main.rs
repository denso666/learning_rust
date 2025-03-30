/*
 * PRINCIPAL FUNCTION
 */
fn main() {
    'main_loop: loop {
        let result: i32 = match menu() {
            1 => sum(get_inputs()),
            2 => sub(get_inputs()),
            3 => mul(get_inputs()),
            4 => div(get_inputs()),
            0 => {
                println!("Thanks, see you soon :)");
                break 'main_loop;
            }
            _ => {
                println!("ERROR: An unexpected error ocurres :(");
                0
            }
        };

        println!("Result: {}", result);
    }
}

fn menu() -> i32 {
    println!("\tCalculator");
    println!("1. Sum");
    println!("2. Sub");
    println!("3. Div");
    println!("4. Mul");
    println!("0. Exit");

    let mut line: String = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("ERROR: an unexpected value was provided");

    let option: i32 = match line.trim().parse() {
        Ok(n) => n,
        Err(e) => {
            println!("ERROR: {}", e);
            -1
        }
    };

    option
}

fn get_inputs() -> [i32; 2] {
    let mut buff0: String = String::new();

    println!("A:");
    std::io::stdin()
        .read_line(&mut buff0)
        .expect("ERROR: an unexpected value was provided");

    let a: i32 = match buff0.trim().parse() {
        Ok(n) => n,
        Err(e) => {
            println!("ERROR: {}", e);
            0
        }
    };

    let mut buff1: String = String::new();
    println!("B:");
    std::io::stdin()
        .read_line(&mut buff1)
        .expect("ERROR: an unexpected value was provided");

    let b = match buff1.trim().parse() {
        Ok(n) => n,
        Err(e) => {
            println!("ERROR: {}", e);
            0
        }
    };

    [a, b]
}

fn sum(arr: [i32; 2]) -> i32 {
    arr[0] + arr[1]
}

fn sub(arr: [i32; 2]) -> i32 {
    arr[0] + arr[1]
}
fn mul(arr: [i32; 2]) -> i32 {
    arr[0] + arr[1]
}
fn div(arr: [i32; 2]) -> i32 {
    arr[0] / arr[1] as i32
}
