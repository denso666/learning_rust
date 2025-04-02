use std::io::Write;

// CONSTANTS
const DEBUG: bool = false;
const ERROR: i32 = -1;
const EXIT: i32 = 0;

// MAIN
fn main() {
    'main_loop: loop {
        let result: i32 = match menu() {
            1 => sum(get_inputs()),
            2 => sub(get_inputs()),
            3 => mul(get_inputs()),
            4 => div(get_inputs()),
            EXIT => {
                println!("Thanks, see you soon :)");
                break 'main_loop;
            }
            _ => {
                if DEBUG {
                    println!("ERROR: An unexpected error ocurres :(");
                }
                ERROR
            }
        };

        if result != ERROR {
            println!("= {}", result);
        }
    }
}

fn menu() -> i32 {
    println!("\n🧮   Calculator   🧮");
    println!("❶ Plus");
    println!("❷ Minus");
    println!("❸ Multiplication");
    println!("❹ Division");
    println!("𝟘 Exit");
    print!("↩︎ ");
    std::io::stdout().flush().unwrap();

    let mut line: String = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("ERROR: an unexpected value was provided");

    let option: i32 = match line.trim().parse() {
        Ok(n) => n,
        Err(e) => {
            println!("ERROR: {}", e);
            ERROR
        }
    };

    option
}

fn get_inputs() -> [i32; 2] {
    let mut buff0: String = String::new();

    print!("A? ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut buff0)
        .expect("ERROR: an unexpected value was provided");

    let a: i32 = match buff0.trim().parse() {
        Ok(n) => n,
        Err(e) => {
            println!("ERROR: {}", e);
            return [0, 0];
        }
    };

    let mut buff1: String = String::new();
    print!("B? ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut buff1)
        .expect("ERROR: an unexpected value was provided");

    let b = match buff1.trim().parse() {
        Ok(n) => n,
        Err(e) => {
            println!("ERROR: {}", e);
            return [0, 0];
        }
    };

    [a, b]
}

// OPERATIONS
fn sum(arr: [i32; 2]) -> i32 {
    arr[0] + arr[1]
}
fn sub(arr: [i32; 2]) -> i32 {
    arr[0] - arr[1]
}
fn mul(arr: [i32; 2]) -> i32 {
    arr[0] * arr[1]
}
fn div(arr: [i32; 2]) -> i32 {
    arr[0] / arr[1]
}
