use crate::tools::tools::remove_existing_indexes;

#[allow(unused)]
#[derive(Debug)]
pub struct List {
    pub owner: String,
    pub items: Vec<Item>,
}
impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.items
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
impl List {
    pub fn new() -> Self {
        Self {
            owner: "Helper Spriggy".to_string(),
            items: Vec::new(),
        }
    }
    fn idx_from_id(&self, id: usize) -> Option<usize> {
        id.checked_sub(1).filter(|&i| i < self.items.len())
    }
    fn reorder(&mut self) -> () {
        // Starred items first!
        self.items.sort_by_key(|item| !item.starred);

        for (idx, items) in self.items.iter_mut().enumerate() {
            items.id = idx + 1;
        }
    }
    pub fn add_item(&mut self, id: usize, content: &str) {
        let item = Item::new(id, content);
        self.items.push(item);
        self.reorder();
    }
    pub fn delete_item(&mut self, id: usize) -> Option<()> {
        let idx = self.idx_from_id(id)?;
        self.items.remove(idx);
        self.reorder();

        Some(())
    }
    pub fn star_something(&mut self, id: usize) -> Option<()> {
        let idx = self.idx_from_id(id)?;
        self.items[idx].starred = true;
        self.reorder();
        Some(())
    }
    //from already formatted file
    pub fn from_str(content: &str) -> std::io::Result<Self> {
        let mut items: Vec<Item> = Vec::new();
        for lines in content.lines().skip(1) {
            let (_, contents) = lines.split_once(";").ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid Item format")
            })?;
            let starred = contents.ends_with("(Starred!)");
            let treated = contents
                .trim()
                .strip_suffix("(Starred!)")
                .unwrap_or(contents.trim())
                .trim();

            items.push(Item {
                id: items.len() + 1,
                content: treated.to_string(),
                starred,
            });
        }

        Ok(Self {
            owner: "Helper Spriggy".to_string(),
            items,
        })
    }
}
//from normal string
impl From<String> for List {
    fn from(value: String) -> Self {
        let treated = remove_existing_indexes(value);
        let items = treated
            .lines()
            .enumerate()
            .map(|(idx, content)| Item::new(idx + 1, content))
            .collect::<Vec<Item>>();

        Self {
            owner: "Helper Spriggy".to_string(),
            items,
        }
    }
}

#[derive(Debug)]
pub struct Item {
    id: usize,
    content: String,
    starred: bool,
}
impl Item {
    fn new(id: usize, content: &str) -> Self {
        Self {
            id,
            content: content.to_string(),
            starred: false,
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.starred {
            return write!(f, "{}: {} (Starred!)", self.id, self.content);
        } else {
            return write!(f, "{}: {}", self.id, self.content);
        }
    }
}
