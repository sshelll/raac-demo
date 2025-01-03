macro_rules! quit_now {
    ($format:expr) => {{
        use colored::Colorize;
        eprintln!("{}", $format.red());
        std::process::exit(1);
    }};
    ($format:expr, $($arg:tt)*) => {{
        use colored::Colorize;
        eprintln!("{}", format!($format, $($arg)*).red());
        std::process::exit(1);
    }};
}

pub(crate) use quit_now;


pub mod sqlx;
pub mod cedar;
