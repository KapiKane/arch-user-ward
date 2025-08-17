fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1) { // index 1 = first argument after program name
        Some(_arg) => println!(""),
        None => println!("No argument provided!"),
    }

if args.contains(&"-r".to_string()) {
    review();
}

if args.contains(&"-i".to_string()) {
    install();
}

if args.contains(&"-s".to_string()) {
    search();
}

}

fn review() {
    println!("u put in r for review");
}

fn install() {
    println!("u put in i for install");
}

fn search() {
    println!("u put in s for search");
}

// R = Review
// S = Download
// s = search
