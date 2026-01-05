pub fn check_email_format(email: &str) -> Option<bool> {
    let (name, domain) = if let Some(split_once) = email.split_once("@") {
        split_once
    } else {
        return Some(false);
    };
    let combined = ['a'..='z', '0'..='9'];

    if name
        .chars()
        .filter(|c| combined.iter().any(|range| range.contains(c)))
        .collect::<Vec<char>>()
        .len()
        != name.len()
    {
        return Some(false);
    }

    let chars: Vec<char> = domain.chars().collect();

    if chars.len() < 3 || chars[chars.len() - 3] != '.' {
        return Some(false);
    }

    if domain.is_empty() {
        return Some(false);
    }

    Some(true)
}
