mod onlinets {    
    pub fn install() {}
    pub fn search() {}
    pub fn review() {}
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1) { // index 1 = first argument after program name
        Some(_arg) => println!(""),
        None => println!("No argument provided!"),
    }

if args.contains(&"-r".to_string()) {
    onlinets::review();
}

if args.contains(&"-i".to_string()) {
    onlinets::install();
}

if args.contains(&"-s".to_string()) {
    onlinets::search();
}

}

// R = Review
// S = Download
// s = search
