use crate::read_config::ConfigFile;
use crate::file_rw::read_license_file;
pub fn codegen(source_string: String, module_name: String, options: ConfigFile) -> String {
    let mut source = source_string;
    source = source.replace("%!%ModuleName%!%", module_name.as_str());
    
    if let Some(license) = options.project.license {
        source = source.replace("%!%License%!%", read_license_file(license).as_str());
    };
    
    if let Some(version) = options.project.version {
        source = source.replace("%!%Version%!%", version.as_str());
    };
    source
}