use std::env;
use std::fs;

fn main() {
    // Get the current directory or the first argument as the directory
    let args: Vec<String> = env::args().collect();
    let dir = if args.len() > 1 { &args[1] } else { "." };

    // Read the directory and iterate over the files
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        let ext_str = ext.to_string_lossy();
                        let folder_name = format!("{}/{}", dir, ext_str);

                        // Create a new folder for the extension if it doesn't exist
                        fs::create_dir_all(&folder_name).unwrap();

                        // Move the file to the appropriate folder
                        let file_name = path.file_name().unwrap().to_string_lossy();
                        let new_path = format!("{}/{}", folder_name, file_name);
                        fs::rename(&path, new_path).unwrap();
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to read directory: {}", dir);
    }
}
