use std::{fs, path::Path};

/*
    Reads a file with a provided path and returns
    the contents of the file as a String
*/

pub fn read() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        /*
           args[0] represents the path of the
           current running executable, the other
           elements are parameters
        */
        panic!("No arguments provided");
    };
    let path: &Path = Path::new::<String>(&args[1]);
    if !path.is_file() {
        panic!("{:?} is not a valid path", path);
    }
    return fs::read_to_string(path).unwrap();
}
