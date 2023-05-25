use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::{fs, panic};
use std::path::{Path, PathBuf};
use std::time::Instant;
use crate::pack_config::{Module, PackConfig};
use crate::package_json::PackageJson;

mod pack_config;
mod package_json;
mod icon_overlay;

#[allow(dead_code)]
fn sample_file() {
    let sample_pack_path = "/Users/parth/Documents/projects/vscode-extension-pack-gen/sample/pack.yaml";
    let sample_output_path = "/Users/parth/Documents/projects/vscode-extension-pack-gen/sample/out";
    let sample_license_path = Some("/Users/parth/Documents/projects/vscode-extension-pack-gen/sample/LICENSE");
    let clear_output_dir = true;
    let install = true;
    let dry_run = true;

    let package_jsons_dir_name = "package-json";
    let extension_pack_dir_name = "extension-pack";
    let vscode_cli_command = "code-insiders";


    // load the yaml into a reader
    let pack = File::open(sample_pack_path).map(|file| {
        let reader = BufReader::new(file);
        let pack: PackConfig = serde_yaml::from_reader(reader)
            .unwrap();
        pack
    }).unwrap();

    println!("> Checking pack for multiple inheritance");
    match pack.find_multiple_inheritance() {
        Ok(_) => { println!("> No multiple inheritance found"); }
        Err(errors) => {
            let mut msg = ":\n> Multiple inheritance found:".to_string();
            for error in errors {
                msg.push_str(&format!("\n\t> {}", error));
            }
            panic!("{msg}");
        }
    }

    // prepare the icons
    let icon_overlay = icon_overlay::IconOverlay::new();
    let common_icon_path = PathBuf::from(&pack.common.icon);
    let icon_dir = PathBuf::from(&pack.common.icon_dir);
    // load the paths from icon_dir and extract the name from the file path `icon_{name}.png`
    let icon_paths = std::fs::read_dir(&icon_dir)
        .expect(&format!("Failed to read directory {}", icon_dir.display().to_string()))
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
        .filter(|path| path.extension().unwrap_or_default() == "png")
        .map(|path| {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let name = file_name.strip_prefix("icon_").unwrap().strip_suffix(".png").unwrap();
            (name.to_string(), path)
        })
        .collect::<HashMap<String, PathBuf>>();

    // convert the pack into a package.json
    let package_jsons: Vec<PackageJson> = pack.into();

    // clear the output directory
    if clear_output_dir && Path::new(&sample_output_path).exists() {
        println!("> Clearing output directory");
        std::fs::remove_dir_all(&sample_output_path).unwrap();
    }

    // write the package.json to the output path
    let mut package_dirs: Vec<(PathBuf, String, String)> = vec![];
    for mut package_json in package_jsons {
        let package_dir = Path::new(&sample_output_path).join(&package_jsons_dir_name).join(&package_json.name);
        let package_json_path = package_dir.clone().join("package.json");
        let package_icon_path = package_dir.clone().join("icon.png");

        // overlay the icon or copy the common icon
        icon_paths.get(&package_json.name).map(|icon_path| {
            icon_overlay.overlay(&common_icon_path, &icon_path, &package_icon_path).unwrap()
        })
            .or_else(|| {
                fs::copy(&common_icon_path, &package_icon_path).unwrap();
                Some(())
            }).unwrap()
        ;

        package_json.icon = format!("icon.png");

        // create the package.json
        if !package_json.overwrite && package_json_path.exists() {
            println!("> Skipping {} already exists at {}", package_json.name, package_json_path.display());
            continue;
        }

        std::fs::create_dir_all(&package_dir).unwrap();

        let package_json_file = File::create(package_json_path).unwrap();
        serde_json::to_writer_pretty(package_json_file, &package_json).unwrap();

        match &sample_license_path {
            Some(license_file_path_str) => {
                let license_file_path = Path::new(license_file_path_str);
                if license_file_path.exists() {
                    // copy the license file
                    let dest_path = package_dir.clone().join("LICENSE");
                    std::fs::copy(license_file_path, dest_path).unwrap();
                }
            }
            None => {}
        }

        package_dirs.push((package_dir, package_json.name, package_json.version));
    }


    // write a script to compile all the extensions
    let mut built_extension_paths: Vec<PathBuf> = vec![];
    let mut build_script = String::new();
    println!("> Generating build script to compile all extensions");
    let package_vsix_dir = Path::new(&sample_output_path).join(&extension_pack_dir_name);
    std::fs::create_dir_all(&package_vsix_dir).unwrap();
    for (package_dir, package_name, package_version) in package_dirs {
        build_script.push_str("#========================================\n");
        build_script.push_str(&format!("# {}\n", package_name));
        build_script.push_str("#========================================\n");

        build_script.push_str(&format!("echo \"Building {}\"\n", package_name));
        build_script.push_str(&format!("cd {} || exit\n", package_dir.display()));

        let package_vsix_path = package_vsix_dir.clone().join(format!("{}-{}.vsix", package_name, package_version));
        build_script.push_str(&format!("vsce package -o {}\n\n", package_vsix_path.display()));

        built_extension_paths.push(package_vsix_path);
    }
    let build_script_path = Path::new(&sample_output_path).join("build.sh");
    let mut build_script_file = File::create(build_script_path).unwrap();
    build_script_file.write_all(build_script.as_bytes()).unwrap();

    // run the build script file
    println!("> Running build script");
    let output = std::process::Command::new("sh")
        .arg("build.sh")
        .current_dir(&sample_output_path)
        .output()
        .expect("> ERROR: Failed to build modules");
    output.stdout.iter().for_each(|byte| print!("{}", *byte as char));
    output.stderr.iter().for_each(|byte| print!("{}", *byte as char));

    // install the extensions
    if install {
        println!("> Generating install script");
        let install_script = built_extension_paths.iter().fold(String::new(), |mut script, ext| {
            script.push_str(&format!("{} --install-extension {}\n", vscode_cli_command, ext.display()));
            script
        });
        let install_script_path = Path::new(&sample_output_path).join("install.sh");
        let mut install_script_file = File::create(install_script_path.clone()).unwrap();
        install_script_file.write_all(install_script.as_bytes()).unwrap();

        if dry_run {
            let path = install_script_path.clone().display().to_string();
            println!("> Dry run, skipping install");
            println!("> Run the following command to install the extensions:");
            println!("> sh {}", path)
        } else {
            println!("> Running install script");
            let output = std::process::Command::new("sh")
                .arg("install.sh")
                .current_dir(&sample_output_path)
                .output()
                .expect("> ERROR: Failed to install modules");
            output.stdout.iter().for_each(|byte| print!("{}", *byte as char));
            output.stderr.iter().for_each(|byte| print!("{}", *byte as char));
        }
    }

    println!("> Done!!!");
}


fn main() {
    let start = Instant::now();
    sample_file();
    let duration = start.elapsed();
    println!("> Took: {:?}", duration);
}
