pub fn print_lexemen(content: String){
    for char in content.chars(){
        if char.is_whitespace(){
            continue
            
        }
        match char {
            '(' => println!("LEFT_PAREN ( null"),

            ')' => println!("RIGHT_PAREN ) null"),

            _ => println!("unknown {}", char),
        }
    }
    println!("EOF  null");
}