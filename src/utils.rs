use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, ErrorKind, Write},
};

// Utility functions for
// file system management
//
// Creates the file
// if it doesn't exist
pub fn create_if_not_exists(path: &str) {
    if let Err(err) = fs::metadata(path) {
        if err.kind() == ErrorKind::NotFound {
            create_file(path);
        }
    }
}
fn create_file(path: &str) -> bool {
    match File::create(path) {
        Ok(_) => true,
        Err(error) => {
            eprintln!("Error creating file :{}", error);
            false
        }
    }
}

// Reads the file
pub fn read_file(path: &str) -> Option<String> {
    create_if_not_exists(path);
    match File::open(path) {
        Ok(file) => {
            let buf = BufReader::new(file);
            Some(buf.lines().map(|ln| ln.unwrap()).collect())
        }
        Err(error) => {
            eprintln!("Error opening file :{}", error);
            None
        }
    }
}

// Writes to file
pub fn write_file(path: &str, data: &str) {
    create_if_not_exists(path);
    match OpenOptions::new().append(true).write(true).open(path) {
        Ok(mut file) => {
            file.write_all(format!("{}\n", data).as_bytes())
                .expect("Error writing to file");
        }
        Err(error) => eprintln!("Error opening file :{}", error),
    }
}

// Parses x.x.x.x:pppp by spliting at ':'
// and returns the ip x.x.x.x
pub fn remove_port(addr: &str) -> &str {
    addr.split(':').next().unwrap()
}
