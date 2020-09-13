use std::env;

pub fn get_port() -> u16 {
    return env::var("PORT")
        .unwrap_or("5000".into())
        .parse::<u16>()
        .expect("Invalid port number");
}
