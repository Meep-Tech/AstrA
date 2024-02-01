use astra::{tests::parser::tokens::tests, utils::log};
use clap::{Parser, Subcommand, ValueEnum};

pub const AUTHOR: &'static str = "Meep.Tech";
const VERSION: &'static str = "0.0.1";
const TEST_DESCRIPTION: &'static str = "Astra Tests";
const TEST_LONG_DESCRIPTION: &'static str = "TEST-MODE:: Used to run tests for the Astra Language";

#[derive(Parser, Debug)]
#[command(
    author = AUTHOR,
    version = VERSION,
    about = TEST_DESCRIPTION,
    long_about = TEST_LONG_DESCRIPTION
)]
struct TestArgs {
    /// A list of parser types to limit tests to
    #[arg(short, long, num_args(0..))]
    types: Vec<String>,

    /// Tags and names for narrowing tests to run
    #[arg(short, long, num_args(0..))]
    names: Vec<String>,

    /// Whether to panic on the first test failure
    #[arg(short, long, default_value_t = false)]
    panic_on_fail: bool,
}

#[derive(Parser, Debug)]
#[command(
    author = AUTHOR,
    version = VERSION,
    about = TEST_DESCRIPTION,
    long_about = TEST_LONG_DESCRIPTION
)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Parse {
        /// Code input to parse
        #[arg(num_args(0..))]
        input: Option<Vec<String>>,

        /// The output format to use
        #[arg(short, long, value_enum)]
        to: Option<Outputs>,

        /// The file/folder path to read input from
        #[arg(short, long)]
        file: Option<String>,

        /// The file to write output to
        #[arg(short, long)]
        out: Option<String>,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Outputs {
    Debug,
    Json,
    SExp,
}

fn main() {
    if cfg!(feature = "test") {
        let input = std::env::args().collect::<Vec<String>>();
        if log::IS_VVV {
            println!("Parsing args from input: {:?}", input);
        }
        let args: TestArgs = TestArgs::parse();
        if log::IS_VVV {
            println!("Computed args from input: {:?}", &args);
        }

        let panic_on_fail = cfg!(feature = "panic-on-fail") || args.panic_on_fail;
        let test_types = args.types;
        let test_tags = args.names;

        let settings = tests::Settings {
            panic_on_fail,
            test_types,
            test_tags,
        };

        println!("Running Tests with Settings: {:?}", &settings);
        astra::parser::init_all();
        tests::run_all_with_settings(&settings);
    } else {
        let input = std::env::args().collect::<Vec<String>>();
        if log::IS_VVV {
            println!("Parsing args from input: {:?}", input);
        }
        let args: Args = Args::parse();
        if log::IS_VVV {
            println!("Computed args from input: {:?}", &args);
        }

        //println!("Not Yet Implemented; Try running with the 'test' feature enabled for now");
    }
}
