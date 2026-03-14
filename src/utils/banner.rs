use colored::Colorize;

pub fn print_banner() {
    let ascii = vec![
        " ██████╗ ██╗  ██╗ ██████╗ ███████╗████████╗ ██████╗ ███████╗██╗███╗   ██╗████████╗",
        "██╔════╝ ██║  ██║██╔═══██╗██╔════╝╚══██╔══╝██╔═══██╗██╔════╝██║████╗  ██║╚══██╔══╝",
        "██║  ███╗███████║██║   ██║███████╗   ██║   ██║   ██║███████╗██║██╔██╗ ██║   ██║   ",
        "██║   ██║██╔══██║██║   ██║╚════██║   ██║   ██║   ██║╚════██║██║██║╚██╗██║   ██║   ",
        "╚██████╔╝██║  ██║╚██████╔╝███████║   ██║   ╚██████╔╝███████║██║██║ ╚████║   ██║   ",
        " ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝    ╚═════╝ ╚══════╝╚═╝╚═╝  ╚═══╝   ╚═╝  ",
    ];

    let terminal_width = term_size::dimensions()
        .map(|(w, _)| w)
        .unwrap_or(120);

    let ascii_width = 86usize;

    let center_pad = |text_len: usize| -> String {
        let pad = terminal_width.saturating_sub(text_len) / 2;
        " ".repeat(pad)
    };

    println!();
    for line in &ascii {
        println!("{}{}", center_pad(ascii_width), line.bright_red().bold());
    }

    let div = "─".repeat(ascii_width);
    println!("{}{}", center_pad(ascii_width), div.truecolor(40, 40, 40));

    let subtitle = "[ AI-Powered OSINT Reconnaissance Framework ]";
    println!("{}{}", center_pad(subtitle.len()), subtitle.bright_red().bold());

    let div2 = "─".repeat(ascii_width);
    println!("{}{}", center_pad(ascii_width), div2.truecolor(40, 40, 40));

    let author_line = "Author: DeveloperAromal   |   github.com/DeveloperAromal/GhostOSINT";
    println!("{}{}", center_pad(author_line.len()), author_line.truecolor(140, 140, 140));

    let tagline = "silent. precise. everywhere.";
    println!("{}{}", center_pad(tagline.len()), tagline.truecolor(80, 80, 80).italic());
    println!();
}