use std::process;
pub fn print_lexemen(content: String){
    let mut nline = 1;
    let mut test_r = true;
    for char in content.chars(){

        if char.is_whitespace(){
            continue

        }
        match char {
            '(' => println!("LEFT_PAREN ( null"),

            ')' => println!("RIGHT_PAREN ) null"),

            '}' => println!("RIGHT_BRACE }} null"),

            '{' => println!("LEFT_BRACE {{ null"),

            '*' => println!("STAR * null"),

            '.' => println!("DOT . null"),

            ',' => println!("COMMA , null"),

            '+' => println!("PLUS + null"),

            '-' => println!("MINUS - null"),

            ';' => println!("SEMICOLON ; null"),
            '\n' =>  nline += 1,

            a => {

                eprintln!("[line 1] Error: Unexpected character: {}",char);
                test_r = false


            }


        }
    }
    println!("EOF  null");
    if test_r == false {
        let exit_code = 65;
        process::exit(exit_code);
    }
}