use tokio;

fn get_resource_path(path: &str) -> std::path::PathBuf {
    let mut r = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    /* The resources are in the top level workspace, but we can't put code there, so we need to go
     * up two levels from our manifest directory to find them.
     */
    r.push("../..");
    r.push(path);
    println!("path: {}", r.display());
    r
}

#[tokio::test]
/* Extract examples from the config manpage.  Examples are between .EX/.EE pairs.  Once extracted,
 * run the config parser over them to make sure they're valid.
 */
async fn man_page_example_configs() {
    use tokio::io::AsyncReadExt as _;

    let mut contents = Default::default();
    tokio::fs::File::open(get_resource_path("man/erbium.conf.5"))
        .await
        .unwrap()
        .read_to_string(&mut contents)
        .await
        .unwrap();
    let mut example: String = Default::default();
    let mut in_example = false;
    let mut examples = 0;

    for line in contents.split("\n") {
        if line == ".EX" {
            example = "".into();
            in_example = true;
        } else if line == ".EE" {
            example = example.replace(
                "\\fIthe-contents-of-the-top-level-addresses-field\\fP",
                "192.0.2.0/24",
            );
            println!("Parsing example: {}", example);
            super::config::load_config_from_string_for_test(&example).unwrap();
            in_example = false;
            examples += 1;
        } else if in_example {
            example += line;
            example += "\n";
        }
    }
    assert_ne!(examples, 0); /* We need to test at least one example */
}

#[tokio::test]
async fn validate_example_config() {
    use tokio::io::AsyncReadExt as _;

    let mut contents = Default::default();
    tokio::fs::File::open(get_resource_path("erbium.conf.example"))
        .await
        .unwrap()
        .read_to_string(&mut contents)
        .await
        .unwrap();
    // If the line is indented, replace the "#" with a " ", keeping the indentation.
    contents = contents.replace("\n#  ", "\n  ");
    // If the line is not indented, then strip the leading "# "
    contents = contents.replace("\n# ", "\n");
    contents = contents.replace(
        "the-contents-of-the-top-level-addresses-field",
        "192.0.2.0/24",
    );
    println!("Parsing contents: {}", contents);
    super::config::load_config_from_string_for_test(&contents).unwrap();
}
