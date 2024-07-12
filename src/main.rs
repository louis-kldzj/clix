mod model;

fn main() {
    let repo = model::load_directory();
    let file = repo.clap_file();
    match file {
        Some(clix_file) => println!("we have a file! {clix_file:?}"),
        None => println!("we don't have a file!"),
    }
}

mod test {
    use super::*;

    fn main() {
        let repo = model::load_directory();
        println!("wow! {repo:?}");
        println!();

        let command = repo.clap();
        println!("clap! {command}");

        let matches = command.get_matches();
        println!("match! {matches:?}");
    }
}
