use astra::{
    parser,
    token::{Code, Source},
};

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    dbg!(&args);

    let cmd = args[1].clone();
    let code;
    if cmd == "parse" || cmd == "p" {
        let code_type = args[2].clone();
        if args[3].starts_with("--") {
            if args[3] == "--code" || args[3] == "--c" {
                code = aggr(4, &args);
            } else if args[3] == "--file" || args[3] == "--f" {
                let file = args[4].clone();
                code = std::fs::read_to_string(file).unwrap();
            } else {
                panic!("Unknown flag: {}", args[3]);
            }
        } else {
            if args[3].ends_with(".axa") {
                let file = args[3].clone();
                code = std::fs::read_to_string(file).unwrap();
            } else {
                code = aggr(3, &args);
            }
        }

        println!(
            "Lexing Code:{}",
            ("\n".to_string() + &code).replace("\n", "\n\t")
        );

        let result = parser::parse(
            &code,
            match code_type.as_str() {
                "axa" => Source::Code(Code::Axa),
                "stx" => Source::Code(Code::Stx),
                "prx" => Source::Code(Code::Prx),
                "blx" => Source::Code(Code::Blx),
                "arc" => Source::Code(Code::Arc),
                "mot" => Source::Code(Code::Mot),
                "cmd" => Source::Code(Code::Cmd),
                _ => panic!(
                    "Unknown code type for AstrA: {}. \nAllowed Types: axa, stx, prx, blx, arc, mot, cmd",
                    code_type, 
                ),
            },
        );
        println!(
            "Result:{}", //\n\n\t------------------------\n{}",
            format!("\n{:#?}", result).replace("\n", "\n\t"),
            // format!("\n{}", color::terms_via_ansi(&code, &terms)).replace("\n", "\n\t"),
        );
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
