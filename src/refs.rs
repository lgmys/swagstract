pub fn find_refs(yaml: &str) -> Vec<String> {
    yaml.lines()
        .filter(|line| line.contains("$ref"))
        .map(|line| line.trim())
        .map(|line| line.replace("'", ""))
        .map(|line| line.replace("./", "#"))
        .map(|line| line.split(" ").last().unwrap_or_default().to_string())
        .collect::<Vec<String>>()
}
