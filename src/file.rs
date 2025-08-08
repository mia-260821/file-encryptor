use std::path::Path;
use walkdir::WalkDir;


pub struct FileVisitor {
    root_path: String,
}

impl FileVisitor {
    pub fn new(root_path: String) -> Self {
        Self {
            root_path,
        }
    }

    fn visit<F>(&self, mut f: F)
    where
        F: FnMut(&Path),
    {
        let path = Path::new(self.root_path.as_str());
        for entry in WalkDir::new(path).into_iter() {
            f(entry.unwrap().path());
        }
    }

    pub fn count_files(&self) -> usize {
        let mut total = 0;
        self.visit_file(|_p| { total += 1 });
        total
    }

    pub fn visit_file<F>(&self, mut f: F)
    where
        F: FnMut(&Path),
    {
        self.visit(|p| {
            if p.is_file() {
                f(p)
            }
        });
    }
}
