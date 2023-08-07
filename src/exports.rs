use rand::Rng;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
pub struct Data {}

// Generates password string
pub fn gen_password(pass_len: usize) -> String {
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    let mut password = String::with_capacity(pass_len);

    let mut rng = rand::thread_rng();

    for _ in 0..pass_len {
        password.push(charset[rng.gen_range(0..charset.len())])
    }

    password
}
