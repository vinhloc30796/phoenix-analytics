use env_logger::Logger;

pub fn get_logger() -> Logger {
    return Logger::from_default_env();
}
