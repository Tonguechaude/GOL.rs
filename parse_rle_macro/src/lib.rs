use proc_macro::TokenStream;
use std::fs;
use std::path::Path;

/// Generate all function for .rle files in a directory
#[proc_macro]
pub fn generate_pattern_functions(input: TokenStream) -> TokenStream {
    let assets_path = input.to_string().trim_matches('"').to_string();

    let rle_files = match find_rle_files(&assets_path) {
        Ok(files) => files,
        Err(e) => {
            let error = format!("compile_error!(\"Failed to read assets directory: {}\");", e);
            return error.parse().unwrap();
        }
    };

    let mut functions = String::new();

    for (fn_name, file_path) in rle_files {
        functions.push_str(&format!(
            "pub fn {}() -> &'static [(i32, i32)] {{\n pattern!(file \"{}\")\n}}\n\n",
            fn_name, file_path
        ));
    }
    functions.parse().unwrap()
}

fn find_rle_files(dir: &str) -> Result<Vec<(String, String)>, std::io::Error> {
    let mut results = Vec::new();
    let path = Path::new(dir);

    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory {} not found", dir)
        ));
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.extension().and_then(|s| s.to_str()) == Some("rle") {
            let file_name = file_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");

            // Convert file name in valid function name
            let fn_name = file_name.replace(['-', ' '],"_").to_lowercase();
            let relative_path = format!("../../{}", file_path.to_string_lossy());

            results.push((fn_name, relative_path));
        }
    }

    Ok(results)
}

