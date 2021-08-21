use crate::file_rw::read_file;
use crate::nio::*;
use crate::read_config::{ConfigFile, CustomFlag, Flags};

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
    let flags: Flags;
    if let Some(f) = options.flags {
        flags = f;
    } else {
        flags = Flags {
            custom_flags: None,
            license_rep: None,
            module_name_rep: None,
            version_rep: None,
        }
    }

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
    source
}

/**********************************************
*****Enumerate over a list of custom flags*****
**********************************************/

pub fn enumerate_custom_flags(
    src: String,
    customs: std::vec::Vec<CustomFlag>,
    args: &[String],
) -> String {
    let mut source = src;
    for custom_flag in customs {
        if custom_flag.source == "arg"
            && source.contains(format!("%!%{}%!%", custom_flag.name).as_str())
        {
            source = arg(custom_flag, args, source);
        } else if custom_flag.source == "str"
            && source.contains(format!("%!%{}%!%", custom_flag.name).as_str())
        {
            source = str(custom_flag, source)
        } else if custom_flag.source == "file"
            && source.contains(format!("%!%{}%!%", custom_flag.name).as_str())
        {
            source = file(custom_flag, source)
        } else if custom_flag.source == "argfile".to_string()
            && source.contains(format!("%!%{}%!%", custom_flag.name).as_str())
        {
            source = argfile(custom_flag, args, source)
        }
    }
    source
}

/*******************************************************************************
*****Replace a custom flag with text specified in a positional CLI argument*****
*******************************************************************************/

pub fn arg(flag: CustomFlag, args: &[String], source: String) -> String {
    let index = flag.replace_with
        .parse::<usize>()
        .unwrap();

    check_args(args, index);
    source.replace(
        format!("%!%{}%!%", flag.name).as_str(),
        args[index - 1].as_str(),
    )
}

/*********************************************************************************
*****Replace a custom flag with a file specified in a positional CLI argument*****
*********************************************************************************/

pub fn argfile(flag: CustomFlag, args: &[String], source: String) -> String {
    let index = flag.replace_with
        .parse::<usize>()
        .unwrap();
    check_args(args, index);
    let path = args[index - 1].clone();
    let file = read_file(&path);
    source.replace(format!("%!%{}%!%", flag.name).as_str(), file.as_str())
}

/******************************************
*****Replace a custom flag with a file*****
******************************************/

pub fn file(flag: CustomFlag, source: String) -> String {
    let path = flag.replace_with;
    let file = read_file(&path);
    source.replace(format!("%!%{}%!%", flag.name).as_str(), file.as_str())
}

/*
*****Replace a custom flag with a string*****
*/

pub fn str(flag: CustomFlag, source: String) -> String {
    source.replace(
        format!("%!%{}%!%", flag.name).as_str(),
        flag.replace_with.as_str(),
    )
}

/*************************************************
*****Check if sufficient arguments are passed*****
*************************************************/

pub fn check_args(args: &[String], index: usize) {
    if index > args.len() {
        red("Insufficient command line arguments for custom flag".to_string());
        panic!()
    }
}