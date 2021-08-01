use crate::file_rw::read_flag_file;
use crate::nio::*;
use crate::read_config::{ConfigFile, CustomFlag};

/*************************************************
*****Generate the output code for a blueprint*****
*************************************************/

pub fn codegen(
    source_string: String,
    module_name: String,
    options: ConfigFile,
    args: &'_ [String],
) -> String {
    let mut source = source_string;
    if let Some(flags) = options.flags {
        source = source.replace(
            match &flags.module_name_rep {
                Some(s) => s.as_str(),
                None => "%!%ModuleName%!%",
            },
            module_name.as_str(),
        );

        source = source.replace(
            match &flags.license_rep {
                Some(s) => s.as_str(),
                None => "%!%License%!%",
            },
            module_name.as_str(),
        );

        source = source.replace(
            match &flags.version_rep {
                Some(s) => s.as_str(),
                None => "%!%Version%!%",
            },
            module_name.as_str(),
        );

        if let Some(customs) = flags.custom_flags {
            source = enumerate_custom_flags(source, customs, args)
        }
        return source;
    }
    source
}

/**********************************************
*****Enumerate over a list of custom flags*****
**********************************************/

fn enumerate_custom_flags(
    src: String,
    customs: std::vec::Vec<CustomFlag>,
    args: &[String],
) -> String {
    let mut source = src;
    for custom_flag in customs {
        if custom_flag.replace_with.to_lowercase().starts_with("arg|")
            && source.contains(format!("%!%{}%!%", custom_flag.name).as_str())
        {
            let index = custom_flag
                .replace_with
                .replacen("arg|", "", 1)
                .parse::<usize>()
                .unwrap();
            if index > args.len() {
                red("Insufficient command line arguments for custom flag".to_string());
                panic!()
            }
            source = source.replace(
                format!("%!%{}%!%", custom_flag.name).as_str(),
                args[index - 1].as_str(),
            )
        } else if custom_flag.replace_with.to_lowercase().starts_with("str|")
            && source.contains(format!("%!%{}%!%", custom_flag.name).as_str())
        {
            source = source.replace(
                format!("%!%{}%!%", custom_flag.name).as_str(),
                custom_flag
                    .replace_with
                    .to_lowercase()
                    .replacen("str|", "", 1)
                    .as_str(),
            );
        } else if custom_flag.replace_with.to_lowercase().starts_with("file|")
            && source.contains(format!("%!%{}%!%", custom_flag.name).as_str())
        {
            let path = custom_flag
                .replace_with
                .to_lowercase()
                .replacen("file|", "", 1);
            let file = read_flag_file(path);
            source = source.replace(
                format!("%!%{}%!%", custom_flag.name).as_str(),
                file.as_str(),
            );
        } else if custom_flag
            .replace_with
            .to_lowercase()
            .starts_with("argfile|")
            && source.contains(format!("%!%{}%!%", custom_flag.name).as_str())
        {
            let index = custom_flag
                .replace_with
                .replacen("argfile|", "", 1)
                .parse::<usize>()
                .unwrap();
            println!("{}", args[0]);
            if index > args.len() {
                red("Insufficient command line arguments for custom flag".to_string());
                panic!();
            }
            let path = args[index - 1].clone();
            let file = read_flag_file(path);
            source = source.replace(
                format!("%!%{}%!%", custom_flag.name).as_str(),
                file.as_str(),
            );
        }
    }
    source
}
