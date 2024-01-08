use astra::parser::lex;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    dbg!(&args);

    let cmd = args[1].clone();
    let code;
    if cmd == "lex" {
        if args[2].starts_with("--") {
            if args[2] == "--code" || args[2] == "--c" {
                code = aggr(3, &args);
            } else if args[2] == "--file" || args[2] == "--f" {
                let file = args[3].clone();
                code = std::fs::read_to_string(file).unwrap();
            } else {
                panic!("Unknown flag: {}", args[2]);
            }
        } else {
            if args[2].ends_with(".axa") {
                let file = args[2].clone();
                code = std::fs::read_to_string(file).unwrap();
            } else {
                code = aggr(2, &args);
            }
        }

        println!(
            "Lexing Code:{}",
            ("\n".to_string() + &code).replace("\n", "\n\t")
        );
        let terms = lex(&code);
        println!("{:#?}", terms);
    } else {
        panic!("Unknown command: {}", cmd);
    }
}

fn aggr(index: usize, args: &Vec<String>) -> String {
    args[index..]
        .join(" ")
        .replace("\\t", "\t")
        .replace("\\n", "\n")
}
