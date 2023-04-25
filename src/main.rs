use std::fs;

fn main() {
    let filename = "foo.txt";
    let result = fs::read_to_string(filename);
    match result {
    Err(error) => {
        println!("**Error. File \"{}\": {}", filename, error);
        return;
    }

    Ok(code) => {

    } 

    }
}
