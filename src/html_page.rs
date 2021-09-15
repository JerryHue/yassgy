use std::fs::File;
use std::path;
use std::io::{BufReader, BufRead, Write, Result};
    
pub struct HtmlPage {
    html_body: String,
}

impl HtmlPage {
    pub fn from<P: AsRef<path::Path>>(file_name: P) -> Result<HtmlPage> {
	let file = File::open(file_name)?;
	let buf_reader = BufReader::new(file);

	let mut lines: Vec<String> = Vec::new();

	for line_result in buf_reader.lines() {
	    lines.push(line_result?);
	}
	    
	let html_body = lines.split(|l| l.trim().is_empty())
	    .filter(|&v| !v.is_empty())
	    .map(|v| v.join("<br />"))
	    .map(|p| format!("<p>{}</p>", p))
	    .fold(String::new(), |mut acc, p| {
		acc.push_str(&p);
		acc
	    });
	    
	Ok(HtmlPage { html_body })
    }

    pub fn write_to_file<P: AsRef<path::Path>>(&self, out_file_name: P) -> Result<()> {
	let mut out_file = File::create(out_file_name)?;

	out_file.write_all(format!("<!doctype html>
<html lang=\"en\">
<head>
    <meta charset=\"utf-8\">
    <title>Filename</title>
    <meta name=\"viewport\" content=\"width=device-width, intial-scale=1\">
</head>
<body>
    {body}
</body>
</html>", body = self.html_body).as_bytes())
    }
}

