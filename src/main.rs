mod model;

fn main() {
    let repo = model::load_directory();
    println!("wow! {repo:?}");
    println!();

    let command = repo.clap();
    println!("clap! {command}");

    let matches = command.get_matches();
    println!("match! {matches:?}");
}
