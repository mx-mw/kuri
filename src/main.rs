use regex::Regex;
use same_file::is_same_file;
use std::{env::args, fs::{read_to_string, File}, path::Path};
use std::io::prelude::*;

type KResult<O> = Result<O, String>;

fn main() -> KResult<()> {
	let mut args = args();

	// Check if we have the correct number of arguments
	if args.len() != 3 {
		return Err("Insufficient Arguments".into());
	}
	args.next();
	let bp_name = args.next().unwrap();
	let gen_name = args.next().unwrap();

	// Find the blueprint and read to a string
	let bp_src = find_bp(bp_name.clone())?;

	// Regex to find the k_ext=x property in the blueprint
	let k_ext_regex = Regex::new("^*k_ext=.$").unwrap();
	let k_ext = match k_ext_regex.find(&bp_src) {
		Some(m) => m.as_str(),
		None    => "k_ext=txt"
	// Split[1] must exist because it either matched our regex or we explicitly defined it
	}.split("=").nth(1).unwrap();

	// Regex to replace {name} with the blueprint name
	let replace_regex = Regex::new("\\{name\\}").unwrap();
	let res = replace_regex.replace_all(&bp_src, &gen_name);

	let filename = format!("{}.{}", bp_name, k_ext);

	let mut save_in = File::create(filename).unwrap();
	save_in.write_all(res.as_bytes()).unwrap();

	Ok(())
}

fn find_bp(name: String) -> KResult<String> {
	let mut path = "./".to_string();
	loop {
		let check = format!("{}.kuri/{}.kbp", path, name);
		if Path::new(&check).exists() {
			break Ok(read_blueprint(check)?);
		}
		// Check if we have reached root, or if `path` does not exist
		if is_same_file(path.clone(), "/").unwrap_or(true) {
			break Err("No .kuri directory found".into());
		}
		path.push_str("../")
	}
}

fn read_blueprint(name: String) -> KResult<String> {
	use std::io::ErrorKind;
	match read_to_string(name) {
		Ok(s) => Ok(s),
		Err(e) => match e.kind() {
			ErrorKind::NotFound => Err("Blueprint not found".into()),
			ErrorKind::PermissionDenied => Err(r"Insufficient permissions.
					Try making your user writeable, or if *absolutely necessary*, run kuri as root."
				.into()),
			_ => Err(e.to_string()),
		},
	}
}
