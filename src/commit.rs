pub fn generate_commit_message(commit_type: String, description: String) -> String {
    format!("{} {}", commit_type, description)
}