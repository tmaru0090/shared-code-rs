use anyhow::anyhow;
use anyhow::Result as Res;
use std::fs::{self, metadata};
use std::{env::args, path::Path, process::Command};

// 言語の種類
#[derive(Debug, Clone)]
enum LanguageType {
    Cpp,
    Rs,
    Py,
    Rb,
    Csharp,
}

// 指定のディレクトリに指定の拡張子があるかどうか返す

fn search_files_with_extension(path: &str, extension: &str) -> bool {
    if let Ok(entries) = fs::read_dir(Path::new(path)) {
        // Check the current directory first
        if search_current_directory_with_extension(extension) {
            return true;
        }

        for entry in entries {
            if let Ok(entry) = entry {
                let file_type = entry.file_type().unwrap();
                if file_type.is_dir() {
                    let dir_path = entry.path();
                    if search_files_with_extension(&dir_path.to_str().unwrap(), extension) {
                        return true;
                    }
                } else {
                    if let Some(file_extension) = entry.path().extension() {
                        if file_extension == extension {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn search_current_directory_with_extension(extension: &str) -> bool {
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_extension) = entry.path().extension() {
                    if file_extension == extension {
                        return true;
                    }
                }
            }
        }
    }
    false
}
// 現在のディレクトリに存在する言語ディレクトリの取得
fn get_language_type() -> Option<LanguageType> {
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    let cpp_file_list = vec![
                        "CMakeLists.txt",
                        "Makefile",
                        "c",
                        "cc",
                        "cpp",
                        "h",
                        "hh",
                        "hpp",
                    ];
                    let rs_file_list = vec!["Cargo.toml", "Cargo.lock", "rs"];
                    let py_file_list = vec![
                        "__init__.py",
                        "setup.py",
                        "requirements.txt",
                        "venv",
                        "virtualenv",
                        "py",
                    ];
                    let rb_file_list = vec!["Gemfile", "Gemfile.lock", "Rakefile", "rb"];
                    let csharp_file_list =
                        vec!["Properties", "App.config", "Web.config", "csproj", "cs"];
                    // c-cppに該当するプロジェクトの場合
                    for cpp in cpp_file_list {
                        if file_name == cpp
                            || search_current_directory_with_extension(cpp)
                            || search_files_with_extension(file_name, cpp)
                        {
                            return Some(LanguageType::Cpp);
                        }
                    }

                    // rustに該当するプロジェクトの場合
                    for rs in rs_file_list {
                        if file_name == rs
                            || search_current_directory_with_extension(rs)
                            || search_files_with_extension(file_name, rs)
                        {
                            return Some(LanguageType::Rs);
                        }
                    }
                    // pythonに該当するプロジェクトの場合
                    for py in py_file_list {
                        if file_name == py
                            || search_current_directory_with_extension(py)
                            || search_files_with_extension(file_name, py)
                        {
                            return Some(LanguageType::Py);
                        }
                    }
                    // rubyに該当するプロジェクトの場合
                    for rb in rb_file_list {
                        if file_name == rb
                            || search_current_directory_with_extension(rb)
                            || search_files_with_extension(file_name, rb)
                        {
                            return Some(LanguageType::Rb);
                        }
                    }
                    // c#に該当するプロジェクトの場合
                    for csharp in csharp_file_list {
                        if file_name == csharp
                            || search_current_directory_with_extension(csharp)
                            || search_files_with_extension(file_name, csharp)
                        {
                            return Some(LanguageType::Csharp);
                        }
                    }
                }
            }
        }
    }
    None
}
// メインエントリー
fn main() -> Res<()> {
    // 引数を取得
    let args: Vec<String> = args().skip(1).collect();
    if args.len() < 1 {
        return Err(anyhow!("You're missing arguments"));
    }
    // デフォルトルートディレクトリ
    let default_root_dir = "/Users/tanukimaru/program/shared-code/";
    // シェアコードディレクトリ
    let shared_code_path = &args[0];
    // 現在のディレクトリ内の使用言語を取得
    if let Some(language_type) = get_language_type() {
        println!("current language_type: {:?}", language_type);
        match language_type {
            LanguageType::Cpp => {
                // 指定されたシェアコードが存在するかどうか
                let path = default_root_dir.to_owned() + &"c-cpp/" + shared_code_path;
                match metadata(path.clone()) {
                    Ok(metadata) => {
                        if metadata.is_file() {
                            println!("The specified path exists: {:?}", path.clone());
                        } else if metadata.is_dir() {
                            return Err(anyhow!("Path exits but is neither no file"));
                        } else {
                            return Err(anyhow!("Path exits but is neither file nor directory"));
                        }
                    }
                    Err(err) => {
                        return Err(anyhow!(err.to_string()));
                    }
                }
                // シェアコードをカレントディレクトリにコピー
                let out = Command::new("powershell")
                    .arg("-Command")
                    .arg("cp")
                    .arg(path.clone())
                    .arg(".")
                    .output()?;
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                println!("{}\n{}", stdout, stderr);
            }
            LanguageType::Rs => {
                let path = default_root_dir.to_owned() + &"rs/" + shared_code_path;
                // 指定されたシェアコードが存在するかどうか
                match metadata(path.clone()) {
                    Ok(metadata) => {
                        if metadata.is_file() {
                            println!("The specified path exists: {:?}", shared_code_path);
                        } else if metadata.is_dir() {
                            return Err(anyhow!("Path exits but is neither no file"));
                        } else {
                            return Err(anyhow!("Path exits but is neither file nor directory"));
                        }
                    }
                    Err(err) => {
                        return Err(anyhow!(err.to_string()));
                    }
                }
                // シェアコードをカレントディレクトリにコピー
                let out = Command::new("powershell")
                    .arg("-Command")
                    .arg("cp")
                    .arg(path.clone())
                    .arg(".")
                    .output()?;
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                println!("{}\n{}", stdout, stderr);
            }
            LanguageType::Py => {
                let path = default_root_dir.to_owned() + &"py/" + shared_code_path;
                // 指定されたシェアコードが存在するかどうか
                match metadata(path.clone()) {
                    Ok(metadata) => {
                        if metadata.is_file() {
                            println!("The specified path exists: {:?}", shared_code_path);
                        } else if metadata.is_dir() {
                            return Err(anyhow!("Path exits but is neither no file"));
                        } else {
                            return Err(anyhow!("Path exits but is neither file nor directory"));
                        }
                    }
                    Err(err) => {
                        return Err(anyhow!(err.to_string()));
                    }
                }
                // シェアコードをカレントディレクトリにコピー
                let out = Command::new("powershell")
                    .arg("-Command")
                    .arg("cp")
                    .arg(path.clone())
                    .arg(".")
                    .output()?;
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                println!("{}\n{}", stdout, stderr);
            }
            LanguageType::Rb => {
                let path = default_root_dir.to_owned() + &"rb/" + shared_code_path;
                // 指定されたシェアコードが存在するかどうか
                match metadata(path.clone()) {
                    Ok(metadata) => {
                        if metadata.is_file() {
                            println!("The specified path exists: {:?}", shared_code_path);
                        } else if metadata.is_dir() {
                            return Err(anyhow!("Path exits but is neither no file"));
                        } else {
                            return Err(anyhow!("Path exits but is neither file nor directory"));
                        }
                    }
                    Err(err) => {
                        return Err(anyhow!(err.to_string()));
                    }
                }
                // シェアコードをカレントディレクトリにコピー
                let out = Command::new("powershell")
                    .arg("-Command")
                    .arg("cp")
                    .arg(path.clone())
                    .arg(".")
                    .output()?;
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                println!("{}\n{}", stdout, stderr);
            }
            LanguageType::Csharp => {
                let path = default_root_dir.to_owned() + &"c#/" + shared_code_path;
                // 指定されたシェアコードが存在するかどうか
                match metadata(path.clone()) {
                    Ok(metadata) => {
                        if metadata.is_file() {
                            println!("The specified path exists: {:?}", shared_code_path);
                        } else if metadata.is_dir() {
                            return Err(anyhow!("Path exits but is neither no file"));
                        } else {
                            return Err(anyhow!("Path exits but is neither file nor directory"));
                        }
                    }
                    Err(err) => {
                        return Err(anyhow!(err.to_string()));
                    }
                }
                // シェアコードをカレントディレクトリにコピー
                let out = Command::new("powershell")
                    .arg("-Command")
                    .arg("cp")
                    .arg(path.clone())
                    .arg(".")
                    .output()?;
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                println!("{}\n{}", stdout, stderr);
            }
            _ => {}
        }
    } else {
        panic!("There are no files or directories identifiable as a specific language in the current directory.");
    }

    Ok(())
}
