
pub enum ItemType {
    File,
    Directory,
}

pub struct Item {
    pub name: String,
    pub selected: bool,
    pub item_type: ItemType,
    pub size: u64,
    pub modified: String,
}

pub struct Column {
    pub name: String,
    pub width: f32,
}

pub struct FilePane {
    pub items: Vec<Item>,
    pub columns: Vec<Column>,
}

impl Default for FilePane {
    fn default() -> Self {
        Self {
            items: vec![
                Item {
                    name: "file1.txt".to_string(),
                    selected: false,
                    item_type: ItemType::File,
                    size: 1024,
                    modified: "2021-01-01 12:00:00".to_string(),
                },
                Item {
                    name: "file directory".to_string(),
                    selected: false,
                    item_type: ItemType::Directory,
                    size: 1337,
                    modified: "2021-01-01 12:00:00".to_string(),
                },
            ],
            columns: vec![
                Column {
                    name: "Icon".to_string(),
                    width: 40.0,
                },
                Column {
                    name: "Name".to_string(),
                    width: 200.0,
                },
                Column {
                    name: "Size".to_string(),
                    width: 100.0,
                },
                Column {
                    name: "Modified".to_string(),
                    width: 200.0,
                },
            ]
        }
    }
}
