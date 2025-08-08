use std::path::Path;
use walkdir::WalkDir;


pub fn visit_all_files<F>(file_path: &str, mut f: F)
where
    F: FnMut(&Path),
{
    let path = Path::new(file_path);

    if path.is_dir() {
        for entry in WalkDir::new(path).into_iter()
        {
            let item = entry.unwrap();
            if item.path().is_file() {
                f(item.path());
            } else {
                println!("{} is not a file", item.path().display());
            }
        }
    } else if path.is_file() {
        f(path);
    } else {
        println!("Not a folder: {}", path.display());
    }
}
