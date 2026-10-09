mod root;

fn main() {
    let entity = root::entry::RootEntity::new(".");

    println!("{} : {}", entity.file_name, entity.file_type);
}
