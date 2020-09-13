use std::env;

pub fn get_port() -> u16 {
    return env::var("PORT")
        .unwrap_or_else(|_| "5000".into())
        .parse::<u16>()
        .expect("Invalid port number");
}


pub fn get_workers() -> usize {
    return env::var("WORKERS")
        .unwrap_or_else(|_| "1".into())
        .parse::<usize>()
        .expect("Invalid worker count");
}
