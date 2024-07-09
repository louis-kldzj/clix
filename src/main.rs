mod repo;

fn main() {
    let repo = repo::load_directory();
    println!("wow! {repo:?}");
}
