fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1) { // index 1 = first argument after program name
        Some(arg) => println!("Got argument: {}", arg),
        None => println!("No argument provided!"),

    if args == "-R" {
        println!("u put in r for review")
    }
    if args == "-S" {
        println!("u put in r for review")
    }
}

// R = Review
// S = Download
// s = search
