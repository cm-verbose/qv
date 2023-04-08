mod reader;

fn main() {
    let source: String = reader::read();
    println!("{}", source); 
}
