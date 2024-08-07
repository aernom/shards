use shards::colorize::Styled;

fn main() {
    println!("{}", "Black on yellow".styled().black().on_yellow());
    println!("{}", "White on blue".styled().white().on_blue());
    println!("{}", "Magenta on green".styled().magenta().on_green());
    println!("{}", "Red on cyan".styled().red().on_cyan());
    println!(
        "{}",
        "rgb on rgb!".styled().rgb(166, 255, 0).on_rgb(55, 85, 88)
    );

    println!(
        "{}",
        "white on blue + bold + underlined"
            .styled()
            .rgb(255, 255, 255)
            .on_blue()
            .bold()
            .underlined()
    );
    println!(
        "{}",
        "black on yellow + dimmed + italic + strikethrough"
            .styled()
            .black()
            .on_yellow()
            .dimmed()
            .italic()
            .strikethrough()
    );
    println!(
        "{}",
        "black on yellow + not dimmed + italic + strikethrough"
            .styled()
            .black()
            .on_yellow()
            .italic()
            .strikethrough()
    );
    println!("{}", "red on white".styled().red().on_rgb(255, 255, 255));
    println!(
        "{}",
        "red on white + inverse"
            .styled()
            .red()
            .on_rgb(255, 255, 255)
            .inverse()
    );
    println!(
        "{}\t<- Hidden",
        "red on white + hidden"
            .styled()
            .red()
            .on_rgb(255, 255, 255)
            .hidden()
    );
    println!(
        "{}",
        "black on white + blinking"
            .styled()
            .rgb(0, 0, 0)
            .on_rgb(255, 255, 255)
            .blinking()
    );
    println!(
        "{}",
        "some style applied + reset_all"
            .styled()
            .rgb(0, 0, 0)
            .on_rgb(255, 255, 255)
            .blinking()
            .reset_all_styles()
    );
}
