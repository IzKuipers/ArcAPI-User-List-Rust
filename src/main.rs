use std::error::Error;

mod args;
mod server;

fn main() {
    let arguments = args::parse_args();

    println!(
        "Server: {} | Authcode: {} | Port: {} | HTTPS: {}",
        arguments.server, arguments.authcode, arguments.port, arguments.is_https
    );

    let _: Result<(), Box<dyn Error>> = server::get_user_list(
        arguments.server,
        arguments.authcode,
        arguments.is_https,
        arguments.port,
    );

    return;
}
