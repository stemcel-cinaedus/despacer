use std::fs;

fn main() {
    let entries = fs::read_dir(".").expect("failed to read directory");

    for entry in entries {
        let entry = entry.expect("failed to read entry");
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        let spaceless = name_str.replace(' ', "");

        if spaceless != name_str.as_ref() {
            fs::rename(entry.path(), spaceless).expect("failed to rename");
        }
    }
}
