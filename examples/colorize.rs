use shards::colorize::Colored;

fn main() {
    println!("{}", "Black on yellow".colored().black().on_yellow());
    println!("{}", "White on blue".colored().white().on_blue());
    println!("{}", "Magenta on green".colored().magenta().on_green());
    println!("{}", "Red on cyan".colored().red().on_cyan());
    println!(
        "{}",
        "rgb on rgb!".colored().rgb(166, 255, 0).on_rgb(55, 85, 88)
    );

    println!(
        "{}",
        "white on blue + bold + underlined"
            .colored()
            .rgb(255, 255, 255)
            .on_blue()
            .bold()
            .underlined()
    );
    println!(
        "{}",
        "black on yellow + dimmed + italic + strikethrough"
            .colored()
            .black()
            .on_yellow()
            .dimmed()
            .italic()
            .strikethrough()
    );
    println!(
        "{}",
        "black on yellow + not dimmed + italic + strikethrough"
            .colored()
            .black()
            .on_yellow()
            .italic()
            .strikethrough()
    );
    println!("{}", "red on white".colored().red().on_rgb(255, 255, 255));
    println!(
        "{}",
        "red on white + inverse"
            .colored()
            .red()
            .on_rgb(255, 255, 255)
            .inverse()
    );
    println!(
        "{}\t<- Hidden",
        "red on white + hidden"
            .colored()
            .red()
            .on_rgb(255, 255, 255)
            .hidden()
    );
    println!(
        "{}",
        "black on white + blinking"
            .colored()
            .rgb(0, 0, 0)
            .on_rgb(255, 255, 255)
            .blinking()
    );
    println!(
        "{}",
        "some style applied + reset_all"
            .colored()
            .rgb(0, 0, 0)
            .on_rgb(255, 255, 255)
            .blinking()
            .reset_all_styles()
    );
}
