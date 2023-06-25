use clap::{arg, App};

fn main() {
    let matches = App::new("my RPN program")
        .version("1.0.0")
        .author("Your name")
        .about("Super awesome sample RPN calculator")
        .arg(arg!([FILE] "Sets the input file to use").required(false))
        .arg(arg!( -v --verbose ..."Sets the level of verbosity").required(false))
        .get_matches();

    match matches.value_of("FILE") {
        Some(file) => println!("The value of file is: {}", file),
        None => println!("No value for file"),
    }

    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", verbose);
}
