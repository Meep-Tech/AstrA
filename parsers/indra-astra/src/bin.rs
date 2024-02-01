use args::Args;
use astra::{parser, tests::parser::tokens::tests};
use getopts::Occur;

const PROGRAM_DESC: &'static str = "Run this program";
const PROGRAM_NAME: &'static str = "program";

const PANIC_ON_FAIL_DESCRIPTION: &'static str =
    "When enabled; the program will panic on the first test failure.";
const TEST_TYPES_DESCRIPTION: &'static str = "The types of the parsers whos tests you want to run";
const TEST_NAMES_DESCRIPTION: &'static str = "The names/tags of the tests you want to run";

fn main() {
    let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);
    if cfg!(feature = "test") {
        args.flag("p", "panic-on-fail", PANIC_ON_FAIL_DESCRIPTION);
        args.option(
            "t",
            "test-type",
            TEST_TYPES_DESCRIPTION,
            "TYPE1 TYPE2",
            Occur::Optional,
            None,
        );
        args.option(
            "t",
            "test-types",
            TEST_TYPES_DESCRIPTION,
            "TYPE1 TYPE2",
            Occur::Optional,
            None,
        );
        args.option(
            "n",
            "test-name",
            TEST_NAMES_DESCRIPTION,
            "NAME1 NAME2",
            Occur::Optional,
            None,
        );
        args.option(
            "n",
            "test-names",
            TEST_NAMES_DESCRIPTION,
            "NAME1 NAME2",
            Occur::Optional,
            None,
        );

        let input = std::env::args().collect::<Vec<String>>();
        println!("Parsing args from input: {:?}", &input);
        let args_result = args.parse(input);
        if let Err(e) = args_result {
            println!("Failed to parse args: {:?}", &e);
            return;
        }

        let panic_on_fail_arg = args.has_value("panic-on-fail");
        let test_types = match args.value_of::<String>("test-type") {
            Ok(test_types) => test_types
                .split(",")
                .flat_map(|s| s.split(" "))
                .map(|s| s.to_string())
                .collect(),
            Err(_) => vec![],
        };
        let test_tags = match args.value_of::<String>("test-name") {
            Ok(test_tags) => test_tags
                .split(",")
                .flat_map(|s| s.split(" "))
                .map(|s| s.to_string())
                .collect(),
            Err(_) => vec![],
        };

        let settings = tests::Settings {
            panic_on_fail: cfg!(feature = "panic-on-fail") || panic_on_fail_arg,
            test_types: test_types.iter().map(|s| s.to_string()).collect(),
            test_tags: test_tags.iter().map(|s| s.to_string()).collect(),
        };

        println!("Running tests with settings: {:?}", &settings);
        parser::init_all();
        tests::run_all_with_settings(&settings);
    } else {
        println!("Not Yet Implemented; Try running with the 'test' feature enabled for now");
    }
}
