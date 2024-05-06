use std::{default, fs::DirEntry, ops::Add};

#[derive(Debug)]
pub struct ZmodTwo<DirEntry> {
    pub items: Vec<DirEntry>,
}

impl<DirEntry> default::Default for ZmodTwo<DirEntry> {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl<DirEntry> From<Vec<DirEntry>> for ZmodTwo<DirEntry> {
    fn from(value: Vec<DirEntry>) -> Self {
        Self { items: value }
    }
}

impl Add for ZmodTwo<DirEntry> {
    type Output = ZmodTwo<DirEntry>;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        let mut diff_files: Vec<DirEntry> = Vec::new();

        let self_items_namespace: &Vec<String> = &self.items.iter().map(file_to_name).collect();
        let rhs_items_namespace: &Vec<String> = &rhs.items.iter().map(file_to_name).collect();

        self.items.append(&mut rhs.items);

        for entry in self.items {
            let entry_name = &entry.file_name().to_str().unwrap().to_string();
            #[allow(clippy::suspicious_arithmetic_impl)]
            if self_items_namespace.contains(entry_name) ^ rhs_items_namespace.contains(entry_name)
            {
                diff_files.push(entry);
            }
        }

        ZmodTwo { items: diff_files }
    }
}

pub fn file_to_name(file: &DirEntry) -> String {
    file.file_name().to_str().unwrap().to_string()
}
