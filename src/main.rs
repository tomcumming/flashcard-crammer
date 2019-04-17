mod db;

fn main() {
    let xs = db::read_list_file::<String>("./data/languages");
    for x in xs {
        println!("{:?}", x);
    }
}
