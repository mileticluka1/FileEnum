use std::env;
use std::fs;
use std::os::unix::fs::MetadataExt;

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // If no arguments were provided, print an error message and exit
    if args.len() < 2 {
        println!("Error: please provide the path to the file as a command line argument");
        return;
    }

    // The first argument is the name of the program, so the file path is the second argument
    let path = &args[1];

    // Use the metadata function from the fs module to get the metadata for the file
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(e) => {
            println!("Error getting metadata for file: {}", e);
            return;
        }
    };

    // Print the file type (e.g. regular file, directory, etc.)
    println!("File type: {:?}", metadata.file_type());

    // Print the permissions of the file (e.g. read, write, execute)
    println!("Permissions: {:o}", metadata.mode() & 0o777);

    // Print the size of the file in bytes
    println!("Size: {} bytes", metadata.len());

    // Print the number of hard links to the file
    println!("Number of hard links: {}", metadata.nlink());

    // Print the user and group IDs of the file owner
    println!("Owned by user ID: {}", metadata.uid());
    println!("Owned by group ID: {}", metadata.gid());

    // Print the time the file was last modified
    println!("Last modified: {}", metadata.mtime());

    // You can also use the fs::read_dir function to enumerate the contents of a directory, if the file is a directory
    if metadata.is_dir() {
        println!("Directory contents:");
        for entry in fs::read_dir(path).unwrap() {
            println!("{}", entry.unwrap().path().display());
        }
    }
}
