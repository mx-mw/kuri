pub struct CodegenOptions {
    module_name: &'static str, 
    license: Option<&'static str>,
    version: Option<&'static str>,   
}

impl CodegenOptions {
    pub fn new (module_name: &'static str, license: Option<&'static str>, version: Option<&'static str>) -> CodegenOptions{
        CodegenOptions {
            module_name: module_name,
            license: license,
            version: version
        }
    }
}

pub fn codegen<'a, 'b>(source_string: String, options: CodegenOptions) -> String {
    let mut source = source_string;
    source = source.replace("%!%ModName%!%", options.module_name);
    
    if let Some(license) = options.license {
        source = source.replace("%!%License%!%", license);
    };
    
    if let Some(version) = options.version {
        source = source.replace("%!%Version%!%", version);
    };
    
    source.to_owned()
}