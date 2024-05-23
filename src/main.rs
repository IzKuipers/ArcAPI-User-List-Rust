use std::process::exit;

use crate::types::info::ServerInfo;
use colored::Colorize;

mod args;
mod server;
mod types;
mod ui;

fn main() {
    ansi_term::enable_ansi_support().unwrap();

    let arguments = args::parse_args();

    let server = arguments.server;
    let authcode = arguments.authcode;
    let is_https = arguments.is_https;
    let port = if is_https { 443 } else { arguments.port };

    println!(
        "\nCalling up {} on port {} ({})...\n",
        server.clone().blue().bold(),
        port.to_string().purple(),
        (if authcode.clone().len() > 0 {
            "Protected"
        } else {
            "Public"
        })
        .yellow()
    );

    let server_info: ServerInfo =
        server::get_server_info(server.clone(), authcode.clone(), is_https, port);
    let user_list = server::get_user_list(server.clone(), authcode.clone(), is_https, port);

    if server_info.revision < 0 {
        println!(
            "{}: {} didn't pick up! Did you give me the right info?",
            "Error".bright_red().bold(),
            server.clone()
        );
        exit(1);
    }

    ui::write_server_info(server_info);
    ui::write_table(user_list.data);
}
