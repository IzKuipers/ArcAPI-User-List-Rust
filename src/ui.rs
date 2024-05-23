use crate::types::{info::ServerInfo, user::User};
use colored::Colorize;

pub fn write_table(user_list: Vec<User>) {
    for i in 0..user_list.len() {
        let user = &user_list[i];

        if !user.acc.enabled {
            continue;
        }

        let index = format!(
            "{:width$}| ",
            i.to_string().truecolor(128, 128, 128),
            width = 5
        );
        let username = format!("{:width$}", user.username.bright_magenta(), width = 30);
        let enabled: String = format!(
            "{:width$}",
            (if user.acc.enabled {
                "Enabled"
            } else {
                "Disabled"
            })
            .cyan(),
            width = 10
        );

        println!("{}{}{}", index, username, enabled);
    }

    println!(
        "\nThis server has {} user(s).",
        &user_list.len().to_string().bright_green()
    )
}

pub fn write_server_info(info: ServerInfo) {
    let protected = format!(
        "{}",
        (if info.protected {
            "protected"
        } else {
            "public"
        })
        .yellow(),
    );
    println!(
        "I'm on the phone with {}, which is a {} server on revision {}.\n",
        info.name.bold(),
        protected,
        info.revision
    );
}
