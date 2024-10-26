fn main() {
    let s3 = String::from("block string");

    {
        let s4 = s3;
        println!("Inside block : {}", s4);
    }
    let divide_result = divide(10, 5);
    println!("{:?}", divide_result);

    let find_char_result = find_char("hello", 'l');
    println!("{:?}", find_char_result);
}

fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(x / y)
    }
}

fn find_char(s: &str, c: char) -> Option<usize> {
    for (i, ch) in s.chars().enumerate() {
        if ch == c {
            return Some(i);
        }
    }
    None
}