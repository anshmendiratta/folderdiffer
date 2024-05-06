use std::{
    fs::{self, read_dir, DirEntry, Metadata},
    path::Path,
    process::Command,
};

use diff_folders::ZmodTwo;

fn main() {
    // Should be three inputs: folder1, folder2, folder_to_dump_in
    // Diff folder1 and folder2 and dump all output files into folder_to_dump_in
    let folders = std::env::args().collect::<Vec<String>>().to_vec();
    let dump_folder_path: String = format!(
        "{}/{}",
        std::env::current_dir().unwrap().display(),
        folders[3]
    );

    let folder_one_files = recurse_all_files(&folders[1]);
    let folder_two_files = recurse_all_files(&folders[2]);

    let folder_diff: Vec<DirEntry> = diff_files(folder_one_files, folder_two_files);
    for item in &folder_diff {
        let final_file_path: String = format!(
            "{}{}",
            &dump_folder_path[..dump_folder_path.len() - 1],
            diff_folders::file_to_name(item)
        );
        Command::new("cp")
            .args([item.path().to_str().unwrap().to_string(), final_file_path])
            .spawn()
            .unwrap();
    }

    let folder_diff_as_sentence: String = {
        let folder_diff_string: Vec<String> = folder_diff
            .iter()
            .map(|file| file.file_name().to_str().unwrap().to_string())
            .collect();
        folder_diff_string.join(", ")
    };

    println!(
        "Copied {} over to {}",
        folder_diff_as_sentence, dump_folder_path
    );
}

fn recurse_all_files(directory: &String) -> Vec<DirEntry> {
    let mut all_files: Vec<DirEntry> = Vec::new();
    let current_directory: String = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let to_search_string = format!("{}/{}", current_directory, directory);
    let directory_to_search = Path::new(&to_search_string);
    let directory_files = match read_dir(directory_to_search) {
        Ok(files) => files.into_iter().map(|entry| entry.unwrap()).collect(),
        Err(_) => Vec::new(),
    };

    for file in directory_files {
        let file_metadata = fs::metadata(file.path()).unwrap();
        if Metadata::is_dir(&file_metadata) {
            let new_path: String = {
                let file_path = file.path();
                let new_path_buffer: &str = file_path.to_str().unwrap();
                let substrings: Vec<&str> = new_path_buffer.split(&current_directory[..]).collect();

                substrings[1].to_string()
            };
            all_files.append(&mut (recurse_all_files(&new_path)));
        } else {
            all_files.push(file)
        };
    }

    all_files
}

fn diff_files(catalog1: Vec<DirEntry>, catalog2: Vec<DirEntry>) -> Vec<DirEntry> {
    let _result_catalog: ZmodTwo<DirEntry> = ZmodTwo::default();
    let catalog1_zmod: ZmodTwo<DirEntry> = ZmodTwo { items: catalog1 };
    let catalog2_zmod: ZmodTwo<DirEntry> = ZmodTwo { items: catalog2 };

    let catalog_result = catalog1_zmod + catalog2_zmod;

    catalog_result.items
}
