/**************
*****Tests*****
***************/

#[cfg(test)]
use crate::codegen::*;
use crate::discover_files::discover_files;
use crate::file_rw::{get_os_path_standard, read_file};
use crate::read_config::{get_directories, ConfigFile, CustomFlag, Meta, Project};

use indoc::indoc;

// test configuration file generation
#[test]
fn config_test() {
    let conf = indoc! {"
        [project]
        project_name=\"TestProject\"
        
        [meta]
        kuri_version=\"1.0\""}
    .to_string();

    let project = Project {
        project_name: "TestProject".to_string(),
        repo: None,
        license: None,
        version: None,
        blueprint_dir: None,
        src_dir: None,
    };

    let meta = Meta {
        kuri_version: "1.0".to_string(),
    };

    let conf_struct = ConfigFile {
        flags: None,
        meta: meta,
        project: project,
        template: None,
    };

    assert_eq!(conf_struct, ConfigFile::read(Some(conf)))
}

// test code generation
#[test]
fn codegen_test() {
    let config_string = indoc! {"
        [project]
        project_name=\"TestProject\"
        
        [meta]
        kuri_version=\"1.0\""}
    .to_string();
    let config = ConfigFile::read(Some(config_string));
    assert_eq!(
        codegen(
            "%!%ModuleName%!%".to_string(),
            "CodegenTest".to_string(),
            config,
            &["".to_string()]
        ),
        "CodegenTest".to_string()
    );
}

// test file discovery
#[test]
fn file_dicovery_test() {
    assert_eq!(
        vec![
            format!(
                "test{0}file_discovery{0}fd0.test.kbp",
                get_os_path_standard()
            ),
            format!(
                "test{0}file_discovery{0}fd1.test.kbp",
                get_os_path_standard()
            ),
            format!(
                "test{0}file_discovery{0}fd2.test.kbp",
                get_os_path_standard()
            )
        ],
        discover_files(format!("test{0}file_discovery", get_os_path_standard()))
    );

    assert_ne!(
        vec![format!(
            "test{0}file_discovery{0}fd0.test.kbp",
            get_os_path_standard()
        )],
        discover_files(format!("test{0}file_discovery", get_os_path_standard()))
    );
}

// test file reading
#[test]
fn read_file_test() {
    // test if the read file function actually works
    assert_eq!(
        read_file(&format!(
            "test{0}read_file{0}fr0.test",
            get_os_path_standard()
        )),
        "Test\n"
    );
    assert_eq!(
        read_file(&format!(
            "test{0}read_file{0}fr1.test",
            get_os_path_standard()
        )),
        "Test Numero Uno\n"
    );
    assert_eq!(
        read_file(&format!(
            "test{0}read_file{0}fr2.test",
            get_os_path_standard()
        )),
        "Test Numero Duo\n"
    );

    // make sure newlines are added
    assert_ne!(
        read_file(&format!(
            "test{0}read_file{0}fr0.test",
            get_os_path_standard()
        )),
        "Test"
    );
    assert_ne!(
        read_file(&format!(
            "test{0}read_file{0}fr1.test",
            get_os_path_standard()
        )),
        "Test Numero Uno"
    );
    assert_ne!(
        read_file(&format!(
            "test{0}read_file{0}fr2.test",
            get_os_path_standard()
        )),
        "Test Numero Duo"
    );
}

// test custom flag enumeration
#[test]
fn cf_enumeration_test() {
    let flags = vec![
        CustomFlag {
            name: "Test1".to_string(),
            replace_with: "str|Test1Tested".to_string(),
        },
        CustomFlag {
            name: "Test2".to_string(),
            replace_with: "arg|1".to_string(),
        },
        CustomFlag {
            name: "Test3".to_string(),
            replace_with: format!(
                "file|test{0}custom_flags{0}file_1.test",
                get_os_path_standard()
            ),
        },
        CustomFlag {
            name: "Test4".to_string(),
            replace_with: "argfile|2".to_string(),
        },
    ];

    assert_eq!(
        enumerate_custom_flags(
            "%!%Test1%!%--%!%Test2%!%--%!%Test3%!%--%!%Test4%!%--".to_string(),
            flags.clone(),
            &[
                "Test2Tested".to_string(),
                format!("test{0}custom_flags{0}file_2.test", get_os_path_standard()),
            ],
        ),
        "test1tested--Test2Tested--ArgfileTest1\n--ArgfileTest2\n--"
    );

    assert_ne!(
        enumerate_custom_flags(
            "%!%Test1%!%--%!%Test2%!%--%!%Test3%!%--%!%Test4%!%--".to_string(),
            flags.clone(),
            &[
                "Test2Tested".to_string(),
                format!("test{0}custom_flags{0}file_2.test", get_os_path_standard()),
            ],
        ),
        "Test1Tested--Test2Tested--ArgfileTest1\n--ArgfileTest2\n--"
    );
}

// test custom flag arg replacement
#[test]
fn cf_arg_test() {
    // test for the first positional argument
    let flag_1 = CustomFlag {
        name: "Test".to_string(),
        replace_with: "arg|1".to_string(),
    };
    assert_eq!(
        arg(
            flag_1.clone(),
            &["It Works?".to_string()],
            "%!%Test%!%".to_string()
        ),
        "It Works?".to_string()
    );
    assert_ne!(
        arg(
            flag_1.clone(),
            &["It Works?".to_string()],
            "%!%Test%!%".to_string()
        ),
        "It doesn't equal".to_string()
    );
    // test for the second positional argument
    let flag_2 = CustomFlag {
        name: "TestNumber2".to_string(),
        replace_with: "arg|2".to_string(),
    };
    assert_eq!(
        arg(
            flag_2.clone(),
            &["It Works?".to_string(), "It works again?".to_string()],
            "%!%TestNumber2%!%".to_string()
        ),
        "It works again?".to_string()
    );
    assert_ne!(
        arg(
            flag_2.clone(),
            &["It Works?".to_string(), "It works again?".to_string()],
            "%!%TestNumber2%!%".to_string()
        ),
        "It still works".to_string()
    );
}

// test custom flag argfile replacement
#[test]
fn cf_argfile_test() {
    // test for the first positional argument
    let flag_1 = CustomFlag {
        name: "Test".to_string(),
        replace_with: "argfile|1".to_string(),
    };
    assert_eq!(
        argfile(
            flag_1.clone(),
            &[format!(
                "test{0}custom_flags{0}file_1.test",
                get_os_path_standard()
            )],
            "%!%Test%!%".to_string()
        ),
        "ArgfileTest1\n".to_string()
    );
    assert_ne!(
        argfile(
            flag_1.clone(),
            &[format!(
                "test{0}custom_flags{0}file_1.test",
                get_os_path_standard()
            )],
            "%!%Test%!%".to_string()
        ),
        "ArgfileTest2\n".to_string()
    );
    // test for the second positional argument
    let flag_2 = CustomFlag {
        name: "TestNumber2".to_string(),
        replace_with: "argfile|2".to_string(),
    };
    assert_eq!(
        argfile(
            flag_2.clone(),
            &[
                format!("test{0}custom_flags{0}file_1.test", get_os_path_standard()),
                format!("test{0}custom_flags{0}file_2.test", get_os_path_standard())
            ],
            "%!%TestNumber2%!%".to_string()
        ),
        "ArgfileTest2\n".to_string()
    );
    assert_ne!(
        argfile(
            flag_2.clone(),
            &[
                format!("test{0}custom_flags{0}file_1.test", get_os_path_standard()),
                format!("test{0}custom_flags{0}file_2.test", get_os_path_standard())
            ],
            "%!%TestNumber2%!%".to_string()
        ),
        "ArgfileTest1\n".to_string()
    );
}

// test custom flag string replacement
#[test]
fn cf_str_test() {
    // test for a random string
    let flag_1 = CustomFlag {
        name: "Test".to_string(),
        replace_with: "str|Tested".to_string(),
    };
    assert_eq!(
        str(flag_1.clone(), "%!%Test%!%".to_string()),
        "tested".to_string()
    );
    assert_ne!(
        str(flag_1.clone(), "%!%Test%!%".to_string()),
        "test works".to_string()
    );
    // test for another string
    let flag_2 = CustomFlag {
        name: "TestNumber2".to_string(),
        replace_with: "str|Test works".to_string(),
    };
    assert_eq!(
        str(flag_2.clone(), "%!%TestNumber2%!%".to_string()),
        "test works".to_string()
    );
    assert_ne!(
        str(flag_2.clone(), "%!%TestNumber2%!%".to_string()),
        "tested".to_string()
    );
}

// test custom flag file replacement
#[test]
fn cf_file_test() {
    // test for file_1.test
    let flag_1 = CustomFlag {
        name: "Test".to_string(),
        replace_with: format!(
            "file|test{0}custom_flags{0}file_1.test",
            get_os_path_standard()
        ),
    };
    assert_eq!(
        file(flag_1.clone(), "%!%Test%!%".to_string()),
        "ArgfileTest1\n".to_string()
    );
    assert_ne!(
        file(flag_1.clone(), "%!%Test%!%".to_string()),
        "ArgfileTest2\n".to_string()
    );
    // test for file_2.test
    let flag_2 = CustomFlag {
        name: "TestNumber2".to_string(),
        replace_with: format!(
            "file|test{0}custom_flags{0}file_2.test",
            get_os_path_standard()
        ),
    };
    assert_eq!(
        file(flag_2.clone(), "%!%TestNumber2%!%".to_string()),
        "ArgfileTest2\n".to_string()
    );
    assert_ne!(
        file(flag_2.clone(), "%!%TestNumber2%!%".to_string()),
        "ArgfileTest1\n".to_string()
    );
}

// test prefix removal
#[test]
fn remove_prefix_test() {
    assert_eq!(remove_prefix("arg|test".to_string(), &"arg|"), "test");
    assert_ne!(remove_prefix("arg|test".to_string(), &"arg|"), "arg|test");
    assert_eq!(
        remove_prefix("argfile|test".to_string(), &"argfile|"),
        "test"
    );
    assert_ne!(
        remove_prefix("argfile|test".to_string(), &"argfile|"),
        "argfile|test"
    );
}

// test getting input & output directories
#[test]
fn get_io_directories_test() {
    let config_1 = new_dummy_cf("Test1", "bps", "source");

    assert_eq!(
        get_directories(&config_1),
        ("bps".to_string(), "source".to_string())
    );
    assert_ne!(
        get_directories(&config_1),
        ("bps".to_string(), "src".to_string())
    );
    assert_ne!(
        get_directories(&config_1),
        ("blueprints".to_string(), "src".to_string())
    );
    assert_ne!(
        get_directories(&config_1),
        ("blueprints".to_string(), "source".to_string())
    );

    let config_2 = new_dummy_cf("Test2", "", "");

    assert_eq!(
        get_directories(&config_2),
        ("blueprints".to_string(), "src".to_string())
    );
    assert_ne!(
        get_directories(&config_2),
        ("bps".to_string(), "src".to_string())
    );

    let config_3 = new_dummy_cf("Test3", "", "source");

    assert_eq!(
        get_directories(&config_3),
        ("blueprints".to_string(), "source".to_string())
    );
    assert_ne!(
        get_directories(&config_3),
        ("blueprints".to_string(), "src".to_string())
    );

    let config_4 = new_dummy_cf("Test4", "", "app");

    assert_eq!(
        get_directories(&config_4),
        ("blueprints".to_string(), "app".to_string())
    );
    assert_ne!(
        get_directories(&config_4),
        ("bluperints".to_string(), "src".to_string())
    );
}

// helper to make a dummy custom flag
fn new_dummy_cf(name: &'static str, bp_dir: &'static str, src_dir: &'static str) -> ConfigFile {
    ConfigFile {
        flags: None,
        meta: Meta {
            kuri_version: "1.0.1".to_string(),
        },
        project: Project {
            project_name: name.to_string(),
            repo: None,
            license: None,
            version: None,
            blueprint_dir: match bp_dir {
                "" => None,
                s => Some(s.to_string()),
            },
            src_dir: match src_dir {
                "" => None,
                s => Some(s.to_string()),
            },
        },
        template: None,
    }
}
