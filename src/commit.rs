pub fn generate_commit_message(commit_type: String, description: String) -> String {
    format!("{} {}", commit_type, description)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_commit_message() {
        let message = generate_commit_message("feat:".to_string(), "Add new button".to_string());
        assert_eq!(message, "feat: Add new button");
    }
}