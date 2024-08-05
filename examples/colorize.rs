use shards::colorize::Colored;

fn main() {
    println!("{}", "Black on yellow".colored().black().on_yellow());
    println!("{}", "White on blue".colored().white().on_blue());
    println!("{}", "Magenta on grees".colored().magenta().on_green());
    println!("{}", "Red on cyan".colored().red().on_cyan());
    println!(
        "{}",
        "rgb on rgb!"
            .colored()
            .rgb(231, 255, 123)
            .on_rgb(137, 34, 75)
    );
}
