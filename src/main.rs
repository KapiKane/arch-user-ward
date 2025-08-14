fn main() {
    let args: Vec<String> = std::env::args().collect();
    let app_query: Vec<String> = std::env::args().collect();

    match args.get(1) {
        Some(arg) => println!("Got argument: {arg}"),
        None => println!("No argument provided!"),
    }

    if app_query.clone().unwrapped() == "-R" {
        println!("u put in r for review")
    }
    if app_query.clone().unwrapped() == "-R" {
        println!("u put in r for review")
    }
}

/* 
-R = review
-S = sync/download
-s = search 
*/