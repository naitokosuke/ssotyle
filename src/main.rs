fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.as_slice() {
        [_, filepath] => {
            println!("{}", filepath);
        }
        _ => {
            eprintln!("Usage: ssotyle <file>");
            std::process::exit(1);
        }
    }
}
