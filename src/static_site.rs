use crate::html_page::HtmlPage;
use std::collections;
use std::fs;
use std::io;
use std::io::Write;
use std::path;

pub struct StaticSite {
    input_root_path: path::PathBuf,
    directory_paths: Vec<path::PathBuf>,
    file_paths: Vec<path::PathBuf>,
    include_index: bool,
}

impl StaticSite {
    pub fn from_directory<P: AsRef<path::Path>>(input_path: P) -> io::Result<StaticSite> {
        let mut dir_paths: Vec<path::PathBuf> = Vec::new();
        let mut file_paths: Vec<path::PathBuf> = Vec::new();

        let mut pending_dirs_to_read: collections::VecDeque<path::PathBuf> =
            collections::VecDeque::new();

        let input_path = input_path.as_ref();
        let root_path = input_path.to_path_buf();
        let mut input_dir_entries = input_path.read_dir()?;

        loop {
            let (mut txt_files, dirs) = StaticSite::get_txt_files_and_dirs(&mut input_dir_entries);

            file_paths.append(&mut txt_files);

            for dir in dirs {
                pending_dirs_to_read.push_back(dir.clone());
                dir_paths.push(dir);
            }

            if let Some(input_path) = pending_dirs_to_read.pop_front() {
                if let Ok(dir_entries) = input_path.read_dir() {
                    input_dir_entries = dir_entries;
                } else {
                    println!("A directory had to be skipped due to some error.");
                }

                continue;
            }

            break;
        }

        Ok(StaticSite {
            input_root_path: root_path,
            directory_paths: dir_paths,
            file_paths,
            include_index: true,
        })
    }

    pub fn from_file<P: AsRef<path::Path>>(input_path: P) -> StaticSite {
        let input_path = input_path.as_ref();
        let root_path: path::PathBuf;

        if let Some(parent) = input_path.parent() {
            root_path = parent.to_path_buf();
        } else {
            root_path = path::PathBuf::from("");
        }

        StaticSite {
            input_root_path: root_path,
            directory_paths: vec![],
            file_paths: vec![input_path.to_path_buf()],
            include_index: false,
        }
    }

    fn get_txt_files_and_dirs(
        dir_path: &mut fs::ReadDir,
    ) -> (Vec<path::PathBuf>, Vec<path::PathBuf>) {
        let mut text_file_paths = Vec::new();
        let mut dir_paths = Vec::new();

        for dir_entry_result in dir_path {
            let dir_entry = dir_entry_result.ok();

            if dir_entry.is_none() {
                println!("A file has been skipped when accessing this file.");
                continue;
            }

            let dir_entry = dir_entry.unwrap();
            let file_metadata = dir_entry.metadata();

            if file_metadata.is_err() {
                println!("A file has been skipped when accessing this file.");
                continue;
            }

            let file_metadata = file_metadata.unwrap();

            if file_metadata.is_dir() {
                dir_paths.push(dir_entry.path());
            } else if file_metadata.is_file() {
                let path: path::PathBuf = dir_entry.path();

                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy().to_lowercase();
                    if "txt" == ext {
                        text_file_paths.push(dir_entry.path());
                    }
                }
            }
        }

        (text_file_paths, dir_paths)
    }

    pub fn create<P: AsRef<path::Path>>(&self, output_dir: P) -> io::Result<()> {
        let mut index: Vec<(String, path::PathBuf)> = vec![];

        let output_folder_path = output_dir.as_ref();

        if output_folder_path.exists() {
            fs::remove_dir_all(output_folder_path)?;
        }

        let dir_builder = fs::DirBuilder::new();
        dir_builder.create(output_folder_path)?;

        for dir in self.directory_paths.iter() {
            let new_dir = output_folder_path.join(
                dir.as_path()
                    .strip_prefix(self.input_root_path.as_path())
                    .unwrap(),
            );
            dir_builder.create(new_dir)?;
        }

        for file in self.file_paths.iter() {
            let page = HtmlPage::new(file)?;

            let relative_file_path = file
                .as_path()
                .strip_prefix(self.input_root_path.as_path())
                .unwrap()
                .with_extension("html");

            index.push((
                page.title().to_owned().to_string(),
                relative_file_path.to_path_buf(),
            ));
            let output_file_path = output_folder_path.join(relative_file_path);

            let output_folder_for_file = output_file_path.parent().unwrap();
            page.write_to_file(output_folder_for_file)?;
        }

        if self.include_index {
            StaticSite::create_index(output_dir, &index[..])
        } else {
            Ok(())
        }
    }

    fn create_index<P: AsRef<path::Path>>(
        output_folder: P,
        index: &[(String, path::PathBuf)],
    ) -> io::Result<()> {
        let mut out_file = fs::File::create(output_folder.as_ref().join("index.html"))?;

        let mut ul = String::from("<ul>");

        for li in index.iter() {
            let href = &li.1;
            let title = &li.0;
            ul.push_str(
                format!(
                    "<li><a href=\"{href}\">{title}</a></li>",
                    href = href.to_str().unwrap(),
                    title = title.as_str()
                )
                .as_str(),
            );
        }
        ul.push_str("</ul>");

        out_file.write_all(
            format!(
                "<!doctype html>
<html lang=\"en\">
<head>
    <meta charset=\"utf-8\">
    <title>Index</title>
    <meta name=\"viewport\" content=\"width=device-width, intial-scale=1\">
</head>
<body>
    {body}
</body>
</html>",
                body = ul
            )
            .as_bytes(),
        )
    }
}
