use clap::{Arg, App};

fn main() {
    let matches = App::new("myapp")
        .version("1.0")
        .author("borbier")
        .about("Does awesome things")
        .arg("-c, --config=[FILE] 'Sets a custom config file'")
        .arg("<INPUT>              'Sets the input file to use'")
        .arg("-v...                'Sets the level of verbosity'")
        .get_matches();
    
    if let Some(i) = matches.value_of("INPUT") {
        println!("Value for input: {}", i);
    }
    
    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    match matches.occurrences_of("v") {
        0 => println!("Verbose mode is off"),
        1 => println!("Verbose mode is kind of on"),
        2 => println!("Verbose mode is on"),
        3 | _ => println!("Don't be crazy"),
    }
}
