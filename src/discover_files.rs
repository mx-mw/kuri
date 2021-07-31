use crate::file_rw::{get_os_path_standard, read_blueprint, write_generated_file};
use crate::codegen::codegen;
use crate::nio::*;
use crate::read_config::ConfigFile;
use glob::glob;

/*********************************
*****Discover blueprint files*****
*********************************/
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

/*********************************************
*****Generate output files for blueprints*****
*********************************************/
pub fn discover_files_loop<'a, 'c, 'b>(
    input_directory: &'a str,
    output_directory: &'a str,
    args: &'b [String],
    config: &'c ConfigFile,
    generated: &mut Vec<String>,
) {
    for i in discover_files(input_directory.to_string()) {
        let bp_file: &str;
        let mut generate_path = String::new();
        let os_path_standard = get_os_path_standard();

        let vec = i.split(os_path_standard).collect::<Vec<&str>>();
        bp_file = match vec.last() {
            None => "",
            Some(f) => f,
        };
        if let Some((_, elements)) = vec.split_last() {
            if let Some((_, first)) = elements.split_first() {
                for j in first {
                    generate_path.push_str(j);
                    generate_path.push(os_path_standard);
                }
            }
        }
        
        let bp_type = bp_file.split('.').collect::<Vec<&str>>()[0];
        let filetype = bp_file.split('.').collect::<Vec<&str>>()[1];
        if bp_type == args[2] {
            let source: String = read_blueprint(format!(
                "{}{}{}{}.{}.kbp",
                input_directory, os_path_standard, generate_path, bp_type, filetype
            ));
            if source != *"" {
                let out = codegen(
                    source,
                    args[3].to_string(),
                    config.clone(),
                    &args[4..].to_vec(),
                );
                match write_generated_file(
                    format!(
                        "{}{}{}{}.{}",
                        output_directory, os_path_standard, generate_path, args[3], filetype
                    ),
                    out,
                    format!("{}{}{}", output_directory, os_path_standard, generate_path),
                ) {
                    Ok(()) => generated.push(format!("Generated {}.{}", args[3], filetype)),
                    Err(e) => red(format!(
                        "Error: {} {}\\{}{}.{}.kbp",
                        e, output_directory, generate_path, bp_type, filetype
                    )),
                };
            }
        }
    }
}
