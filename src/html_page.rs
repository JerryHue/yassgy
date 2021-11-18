use std::borrow;
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::io::{BufRead, Write};
use std::path;

#[derive(Debug, Clone, Copy)]
enum ErrorKind {
    PathNameNotTextFile,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::PathNameNotTextFile => {
                write!(f, "path to file is not txt file")
            }
        }
    }
}

impl error::Error for ErrorKind {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct HtmlPage {
    file_name: path::PathBuf,
    title_from_file: Option<String>,
    html_body: String,
    language_tag: String,
}

impl HtmlPage {
    fn is_path_to_text_file<P: AsRef<path::Path>>(
        file_name: &P,
    ) -> io::Result<()> {
        let file_name_ref = file_name.as_ref();

        if let Some(extension) = file_name_ref.extension() {
            let extension = extension.to_string_lossy().to_lowercase();
            if extension == "txt" {
                return Ok(());
            }
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            ErrorKind::PathNameNotTextFile,
        ))
    }

    pub fn new<P: AsRef<path::Path>>(
        file_name: &P,
        language_tag: &str,
    ) -> io::Result<HtmlPage> {
        HtmlPage::is_path_to_text_file(&file_name)?;

        let file = fs::File::open(file_name)?;

        let buf_reader = io::BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();

        for line_result in buf_reader.lines() {
            lines.push(line_result?);
        }

        let (title_from_file, lines) = HtmlPage::get_title_and_body(&lines[..]);

        let html_body = lines
            .split(|l| l.trim().is_empty())
            .filter(|&v| !v.is_empty())
            .map(|v| v.join("<br />"))
            .map(|p| format!("<p>{}</p>", p))
            .fold(String::new(), |mut acc, p| {
                acc.push_str(&p);
                acc
            });

        Ok(HtmlPage {
            file_name: Box::new(file_name.as_ref()).to_path_buf(),
            title_from_file,
            html_body,
            language_tag: language_tag.to_owned(),
        })
    }

    fn get_title_and_body(lines: &[String]) -> (Option<String>, &[String]) {
        let mut start = 0;
        let mut end = 0;

        for (i, line) in lines.iter().enumerate() {
            if !line.trim().is_empty() {
                start = i;
                break;
            }
        }

        for i in (0..lines.len()).rev() {
            if !lines[i].trim().is_empty() {
                end = i + 1;
                break;
            }
        }

        let actual_lines = &lines[start..end];

        if actual_lines.len() > 3 {
            let first_line = &actual_lines[0];
            let second_line = &actual_lines[1];
            let third_line = &actual_lines[2];

            if second_line.is_empty() && third_line.is_empty() {
                return (
                    Some(first_line.clone()),
                    &actual_lines[(start + 3)..=end],
                );
            }
        }

        (None, actual_lines)
    }

    pub fn title(&self) -> borrow::Cow<str> {
        if let Some(title) = &self.title_from_file {
            return borrow::Cow::from(title.as_str());
        }

        self.file_name.file_stem().expect("").to_string_lossy()
    }

    pub fn write_to_file<P: AsRef<path::Path>>(
        &self,
        dir_path: P,
    ) -> io::Result<()> {
        let output_path = dir_path
            .as_ref()
            .join(self.file_name.file_name().unwrap())
            .with_extension("html");

        let mut out_file = fs::File::create(output_path)?;

        if self.title_from_file == None {
            out_file.write_all(
                format!(
                    "<!doctype html>
<html lang=\"{lang_tag}\">
<head>
    <meta charset=\"utf-8\">
    <title>{title}</title>
    <meta name=\"viewport\" content=\"width=device-width, intial-scale=1\">
</head>
<body>
    {body}
</body>
</html>",
                    lang_tag = self.language_tag,
                    title = self.title(),
                    body = self.html_body
                )
                .as_bytes(),
            )
        } else {
            out_file.write_all(
                format!(
                    "<!doctype html>
<html lang=\"{lang_tag}\">
<head>
    <meta charset=\"utf-8\">
    <title>{title}</title>
    <meta name=\"viewport\" content=\"width=device-width, intial-scale=1\">
</head>
<body>
    <h1>{title}</h1>
    {body}
</body>
</html>",
                    lang_tag = self.language_tag,
                    body = self.html_body,
                    title = self.title()
                )
                .as_bytes(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Result as IoResult;

    #[test]
    fn file_name_refers_to_text_file() -> IoResult<()> {
        let file_name = "file.txt";
        HtmlPage::is_path_to_text_file(&file_name)
    }

    #[test]
    fn file_name_does_not_refer_to_text_file() {
        let file_name = "file.html";
        let error = HtmlPage::is_path_to_text_file(&file_name);

        assert!(error.is_err());
    }

    #[test]
    fn file_name_does_not_have_extension() {
        let file_name = "file";
        let error = HtmlPage::is_path_to_text_file(&file_name);

        assert!(error.is_err());
    }

    #[test]
    fn valid_file_name_does_not_exist() {
        let file_name = "file.txt";
        let error = HtmlPage::new(&file_name, &"en-CA");

        assert!(error.is_err());
    }

    #[test]
    fn empty_file_is_read() {
        let file_name = "testfiles/txt/empty.txt";
        let page = HtmlPage::new(&file_name, &"en-CA").unwrap();
        page.write_to_file("testfiles/txt").unwrap();

        let actual_html =
            fs::read_to_string("testfiles/txt/empty.html").unwrap();

        assert_eq!(
            actual_html,
            "<!doctype html>
<html lang=\"en-CA\">
<head>
    <meta charset=\"utf-8\">
    <title>empty</title>
    <meta name=\"viewport\" content=\"width=device-width, intial-scale=1\">
</head>
<body>
    
</body>
</html>"
        );
    }
}
