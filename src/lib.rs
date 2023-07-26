#[macro_export]
macro_rules! print_version_info {
    ($name: expr, $version: expr) => {{
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 && (args[1] == "--version" || args[1] == "-v") {
            let name = $name;
            let version = $version;
            let info = format!(
                "\x1B[1m{name}\x1B[0m version: \x1B[1m{name}\x1B[0m/{version} ({})",
                std::env::consts::OS,
            );
            println!("{}", info);
            std::process::exit(0);
        }
    }}
}

#[macro_export]
macro_rules! env_vars {
    ($(($key:expr, $value:expr, $must_be:expr)),+ $(,)?) => {{
        let mut missing_envs: Vec<&str> = vec![];
        $(
            let env_val = std::env::var($key).unwrap_or_default();
            if !env_val.is_empty() {
                *$value = env_val;
            } else if $must_be {
                missing_envs.push($key);
            }
        )+
        if !missing_envs.is_empty() {
            println!("\x1B[1m\x1B[4mUSAGE:\x1B[0m Must be set \x1B[1m{}\x1B[0m", missing_envs.join(", "));
            std::process::exit(1);
        }
    }}
}

#[macro_export]
macro_rules! logger_log {
    ($context: expr, $message: expr) => {{
        let context = $context;
        let message = $message;
        let info = format!(
            "\x1B[32m[Rocket] - \x1B[0m{}\x1B[32m LOG \x1B[33m[{context}]\x1B[32m {message}\x1B[0m",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        );
        println!("{}", info);
    }}
}