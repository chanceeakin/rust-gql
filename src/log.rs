extern crate slog;
extern crate slog_async;
extern crate slog_term;

use slog::*;

pub struct Log {
    log: slog::Logger::root(drain, o!("version" => "0.5")),
}
