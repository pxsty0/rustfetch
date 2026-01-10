use owo_colors::OwoColorize;

use crate::info::Info;
use crate::logo::rust_logo;

pub fn print_with_logo(info: &Info) {
    let lines = rust_logo();

    let label = |s: &str| format!("{}", s.bold());

    let fields = vec![
        format!(
            "{}{}{} {}{}{}",
            info.user.bold(),
            "@".dimmed(),
            info.host.bold(),
            "(".dimmed(),
            info.os_name.as_str().dimmed(),
            ")".dimmed()
        ),
        format!("{} {}", label("OS"), info.os_pretty),
        format!("{} {}", label("Kernel"), info.kernel),
        format!("{} {}", label("Uptime"), info.uptime),
        format!("{} {}", label("Shell"), info.shell),
        format!("{} {}", label("Terminal"), info.terminal),
        format!("{} {}", label("Packages"), info.packages),
        format!("{} {}", label("Resolution"), info.resolution),
        format!("{} {}", label("DE/WM"), info.de_wm),
        format!("{} {}", label("CPU"), info.cpu),
        format!("{} {}", label("GPU"), info.gpu),
        format!("{} {}", label("Memory"), info.memory),
        color_blocks(),
    ];

    align_and_print(&lines, &fields);
}

fn align_and_print(logo: &[String], fields: &[String]) {
    let padding = 2;
    let max_logo_lines = logo.len();
    let max_field_lines = fields.len();
    let total = max_logo_lines.max(max_field_lines);

    for i in 0..total {
        let logo_part = if i < max_logo_lines {
            logo[i].as_str()
        } else {
            ""
        };
        let info_part = if i < max_field_lines {
            fields[i].as_str()
        } else {
            ""
        };
        if info_part.is_empty() {
            println!("{}", logo_part);
        } else if logo_part.is_empty() {
            println!("{:width$}{}", "", info_part, width = 22 + padding);
        } else {
            println!(
                "{logo}{sp}{info}",
                logo = logo_part,
                sp = " ".repeat(padding),
                info = info_part
            );
        }
    }
}

fn color_blocks() -> String {
    let blocks = [
        (255, 59, 48),   // red
        (255, 149, 0),   // orange
        (255, 204, 0),   // yellow
        (52, 199, 89),   // green
        (0, 122, 255),   // blue
        (88, 86, 214),   // indigo
        (175, 82, 222),  // purple
        (142, 142, 147), // gray
    ];
    blocks
        .iter()
        .map(|(r, g, b)| "■".to_string().truecolor(*r, *g, *b).to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
