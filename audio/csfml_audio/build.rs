use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    //拷贝Dll到可执行文件所在目录
    if let Ok(csfml_home) = env::var("CSFML_HOME") {
        let mut csfml_home = PathBuf::from(csfml_home);
        csfml_home.push("bin");

        for entry in fs::read_dir(csfml_home).expect("Can't read DLL dir") {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();

            let mut target_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
            target_dir.push("target");
            target_dir.push(env::var("PROFILE").unwrap());

            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();

                if file_name.ends_with(".dll") {
                    target_dir.push(file_name);
                    fs::copy(&entry_path, target_dir.as_path()).expect("Can't copy from DLL dir");
                }
            }
        }
    }
}
