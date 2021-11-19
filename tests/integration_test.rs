use std::ffi::OsString;
use std::fs;
use yassgy::static_site::StaticSite;

#[test]
fn create_static_site_from_a_single_file() {
    let static_site = StaticSite::from_file(&"testfiles/txt/normal.txt");

    static_site
        .create(&"testfiles/txt/normal", &"en-CA")
        .unwrap();

    let actual_html =
        fs::read_to_string("testfiles/txt/normal/normal.html").unwrap();

    assert_eq!(
        actual_html,
        "<!doctype html>
<html lang=\"en-CA\">
<head>
    <meta charset=\"utf-8\">
    <title>normal</title>
    <meta name=\"viewport\" content=\"width=device-width, intial-scale=1\">
</head>
<body>
    <p>This is a normal fie, which only contains a single paragraph.</p>
</body>
</html>"
    );
}

#[test]
fn create_static_site_from_a_directory() {
    let static_site =
        StaticSite::from_directory(&"testfiles/dirs/normal").unwrap();

    static_site
        .create(&"testfiles/dirs/normal_out", &"en-CA")
        .unwrap();

    let file_entries: Vec<OsString> =
        fs::read_dir(&"testfiles/dirs/normal_out")
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();

    assert_eq!(file_entries.len(), 3);
    assert!(file_entries.iter().any(|name| name == "index.html"));
    assert!(file_entries.iter().any(|name| name == "file1.html"));
    assert!(file_entries.iter().any(|name| name == "file2.html"));

    let index_contents =
        fs::read_to_string("testfiles/dirs/normal_out/index.html").unwrap();

    assert_eq!(index_contents,
    "<!doctype html>
<html lang=\"en-CA\">
<head>
    <meta charset=\"utf-8\">
    <title>Index</title>
    <meta name=\"viewport\" content=\"width=device-width, intial-scale=1\">
</head>
<body>
    <ul><li><a href=\"file1.html\">file1</a></li><li><a href=\"file2.html\">file2</a></li></ul>
</body>
</html>");

    let file1_contents =
        fs::read_to_string("testfiles/dirs/normal_out/file1.html").unwrap();

    assert_eq!(
        file1_contents,
        "<!doctype html>
<html lang=\"en-CA\">
<head>
    <meta charset=\"utf-8\">
    <title>file1</title>
    <meta name=\"viewport\" content=\"width=device-width, intial-scale=1\">
</head>
<body>
    <p>This is file1 text.</p>
</body>
</html>"
    );

    let file2_contents =
        fs::read_to_string("testfiles/dirs/normal_out/file2.html").unwrap();

    assert_eq!(
        file2_contents,
        "<!doctype html>
<html lang=\"en-CA\">
<head>
    <meta charset=\"utf-8\">
    <title>file2</title>
    <meta name=\"viewport\" content=\"width=device-width, intial-scale=1\">
</head>
<body>
    <p>This is file2 text.</p>
</body>
</html>"
    );
}
