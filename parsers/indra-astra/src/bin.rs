use indra_astra::{
    parser::{self},
    tests::parser::tokens::tests,
};

fn init() {
    parser::init_all();
}

fn main() {
    init();
    if cfg!(feature = "test") {
        let settings = tests::Settings {
            panic_on_fail: cfg!(feature = "panic-on-fail"),
        };
        println!("Running tests with settings: {:?}", &settings);

        tests::run_all_with_settings(&settings);
    }
}
