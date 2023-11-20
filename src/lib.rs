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

    fn add(self, rhs: Self) -> Self::Output {
        let mut diff_files: Vec<DirEntry> = Vec::new();

        let self_items_namespace: &Vec<String> = &self.items.iter().map(|entry| file_to_name(entry)).collect();
        let rhs_items_namespace: &Vec<String> = &rhs.items.iter().map(|entry| file_to_name(entry)).collect();

        dbg!(&self_items_namespace, &rhs_items_namespace);

        for entry in rhs.items {
            let entry_name = &entry.file_name().to_str().unwrap().to_string();
            if self_items_namespace.contains(&entry_name) ^ rhs_items_namespace.contains(&entry_name) == true {
                diff_files.push(entry);
            }
        }

        dbg!(&diff_files);

        ZmodTwo {
            items: diff_files
        }
    }
}

pub fn file_to_name(file: &DirEntry) -> String {
    file.file_name().to_str().unwrap().to_string()
}

pub fn count_occurences<T>(object: &T, to_check: Vec<T>) -> usize
where
    T: PartialEq,
{
    to_check.into_iter().filter(|item| *item == *object).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_occurences() {
        let check_vector = vec![1, 2, 3, 4, 2];
        let check_number = 2;
        assert_eq!(count_occurences(&check_number, check_vector), 2);
    }
}
