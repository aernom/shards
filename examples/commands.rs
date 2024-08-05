use shards::commands::open;

fn main() {
    if let Err(err) = open("mailto:andrea.montorio@gmail.com") {
        println!("{err}");
    }
}
