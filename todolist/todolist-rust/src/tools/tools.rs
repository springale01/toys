pub fn remove_existing_indexes(content: String) -> String {
    content
        .lines()
        .filter_map(|line| line.split_once(":").map(|(_, y)| y.trim().to_string()))
        .collect::<Vec<_>>()
        .join("\n")
}
