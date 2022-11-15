use alang_lib::{
    interpreter::{self, Environment},
    lexer::tokenize,
    parser::Parser,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let mut env = Environment::new();

    if args.len() > 1 {
        let filename = &args[1];

        // read the file
        let source_code = std::fs::read_to_string(filename).expect("Could not read file");

        // parse the file
        // let tokens = tokenize(&source_code)?;

        let program = Parser::produce_ast(&source_code)?;

        let output = interpreter::run(&program, &mut env)?;

        println!("{:#?}", output);

        std::process::exit(0);
    }

    let tokens = tokenize("hello 2.15 + 102.50\n\"Some String\" rawr 12 '\n'")?;

    println!("{:#?}", tokens);

    Ok(())
}
