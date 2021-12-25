use termion::color;

fn main() {
    let leaf = "*";
    let trunk = "#";
    let greetings = "Happy XMAS!";
    let top_size = 13;
    let bottom_size = 31;
    let trunk_size = 3;
    let leaf_color = color::LightGreen;
    let trunk_color = color::Rgb(153, 51, 51);
    let greetings_bg = color::Red;
    let greetings_color = color::White;

    // top
    for i in (1..=top_size).step_by(2) {
        println!(
            "{}{}{}",
            color::Fg(leaf_color),
            " ".repeat((top_size + 1 - i) / 2 + (bottom_size - top_size) / 2),
            leaf.repeat(i)
        );
    }
    // bottom
    for i in (7..=bottom_size).step_by(2) {
        println!(
            "{}{}{}",
            color::Fg(leaf_color),
            " ".repeat((bottom_size + 1 - i) / 2),
            leaf.repeat(i)
        );
    }
    // trunk
    for _ in 0..2 {
        println!(
            "{}{}{}",
            color::Fg(trunk_color),
            " ".repeat(bottom_size / 2 - trunk_size / 2),
            trunk.repeat(trunk_size)
        );
    }
    // Greetings
    println!(
        "{}{}{} {} {}{}",
        " ".repeat(bottom_size / 2 - greetings.len() / 2 - 1),
        color::Fg(greetings_color),
        color::Bg(greetings_bg),
        greetings,
        color::Fg(color::Reset),
        color::Bg(color::Reset),
    );
}
