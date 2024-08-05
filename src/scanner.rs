use std::process;

pub fn print_lexemen(content: String) {
    let mut string_stg: [char; 2] = [' ', ' '];
    let mut test_r = true;
    let mut rep_op: i32 = 0;
    for char in content.chars() {
        if char.is_whitespace() {
            continue
        }
        if rep_op == 1 {
            match string_stg[0] {
                '<' => println!("LESS < null"),
                '>' => println!("GREATER >  null"),
                '=' => println!("EQUAL = null"),
                
                _ => {}
            }
            rep_op = 0
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

            '='|'<'|'>'|'!' =>
                if string_stg[0] != ' ' {

                    match string_stg[0] {
                        '<' => println!("LESS_EQUAL <= null"),
                        '>' => println!("GREATER_EQUAL <= null"),
                        '=' => println!("EQUAL_EQUAL == null"),
                        '!' => println!("BANG_EQUAL != null"),
                        _ => {}
                    }
                    string_stg = [' ', ' '];

                }
                else {
                    string_stg[0] = char;
                    rep_op = 1;
                },

            a => {
                eprintln!("[line 1] Error: Unexpected character: {}", char);
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
