#[test]
fn test_readme_deps() {
    version_sync::assert_markdown_deps_updated!("README.md");
}

#[test]
fn test_html_root_url() {
    version_sync::assert_html_root_url_updated!("src/main.rs");
}

#[test]
fn test_readme_changelog() {
    version_sync::assert_contains_regex!(
        "README.md",
        r"^### Version {version} — \d{2}\.\d{2}\.\d{4}$"
    );
}