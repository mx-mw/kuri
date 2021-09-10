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
    enumerate_custom_flags(source, options.flags.flags, args)
}

/**********************************************
*****Enumerate over a list of custom flags*****
**********************************************/

pub fn enumerate_custom_flags(
    src: String,
    customs: std::vec::Vec<CustomFlag>,
    args: &[String],
) -> String {
    println!("Codegen");
    let mut source = src;
    for custom_flag in customs {
        println!("Codegen");
        if custom_flag.source.as_str() == "arg"
            && source.contains(format!("%!%{}%!%", custom_flag.name).as_str())
        {
            println!("Generating flag from arg {}", custom_flag.replace_with);
            source = arg(custom_flag, args, source);
        } else if custom_flag.source.as_str() == "str"
            && source.contains(format!("%!%{}%!%", custom_flag.name).as_str())
        {
            source = str(custom_flag, source)
        } else if custom_flag.source.as_str() == "file"
            && source.contains(format!("%!%{}%!%", custom_flag.name).as_str())
        {
            source = file(custom_flag, source)
        } else if custom_flag.source.as_str() == "argfile".to_string()
            && source.contains(format!("%!%{}%!%", custom_flag.name).as_str())
        {
            source = argfile(custom_flag, args, source)
        } else {
            println!("{:?}, {}", custom_flag.source, "arg");
        }
    }
    source
}

/*******************************************************************************
*****Replace a custom flag with text specified in a positional CLI argument*****
*******************************************************************************/

pub fn arg(flag: CustomFlag, args: &[String], source: String) -> String {
    let index = flag.replace_with.parse::<usize>().unwrap();

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
    let index = flag.replace_with.parse::<usize>().unwrap();
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
