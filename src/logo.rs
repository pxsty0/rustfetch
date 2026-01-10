use owo_colors::OwoColorize;

pub fn rust_logo() -> Vec<String> {
    let base = vec![
        "                         ",
        "        @@@@@@@@@@       ",
        "    @ @@@@ @@@  @@@@@@   ",
        "   @@@@            @@@@  ",
        "  @@@@@@@@@@@@@@@@  @@@@ ",
        "  @  @ @@@@    @@@@ @ @@ ",
        "  @@   @@@@@@@@@@@@  @@@ ",
        " @@@   @@@@@@@@@@@    @@@",
        " @@@   @@@@    @@@@  @@@@",
        "  @@@@@@@@@@@  @@@@@@@@@ ",
        "   @@@              @@@  ",
        "    @@@ @        @ @@@   ",
        "      @@@@@@@@@@@@@@      ",
        "        @ @ @@@@ @        ",
    ];

    base.into_iter()
        .enumerate()
        .map(|(i, l)| match i % 4 {
            0 => l.truecolor(222, 165, 132).to_string(),
            1 => l.truecolor(201, 132, 88).to_string(),
            2 => l.truecolor(160, 93, 63).to_string(),
            _ => l.truecolor(120, 69, 45).to_string(),
        })
        .collect::<Vec<_>>()
}
