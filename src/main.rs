mod args;

fn main() {
    let arguments = args::parse_args();

    println!(
        "Server: {} | Authcode: {}",
        arguments.server, arguments.authcode
    );
}
