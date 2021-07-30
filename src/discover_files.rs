use glob::glob;

pub fn discover_files(search_dir: String) -> Vec<String> {
    let mut paths_vec: Vec<String> = Vec::new();

    for i in glob(format!("{}/**/*.kbp", search_dir.as_str()).as_str())
        .expect("Failed to read Glob pattern")
    {
        match i {
            Ok(s) => paths_vec.push(format!("{}", s.display())),
            Err(e) => panic!("{}", e),
        }
    }

    paths_vec
}
