pub fn print_lexemen(content: String){
    for char in content.chars(){
        if char.is_whitespace(){
            continue
            
        }
        match char {
            '(' => println!("LEFT_PAREN ( null"),

            ')' => println!("RIGHT_PAREN ) null"),

            '}' => println!("RIGHT_BRACE }} null"),

            '{' => println!("LEFT_BRACE {{ null"),

            _ => println!("unknown {}", char),
        }
    }
    println!("EOF  null");
}