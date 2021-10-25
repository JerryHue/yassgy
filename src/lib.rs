mod static_site;
mod html_page;
use static_site::StaticSite;
use std::path::Path;
use std::io::Result;

enum InputFilePath<'a> {
    RegularFilePath(&'a Path),
    DirectoryPath(&'a Path),
}

impl <'a> InputFilePath<'a> {
    pub fn from_path<P: AsRef<Path>>(input_path: &'a P) -> Option<InputFilePath<'a>> {
	use InputFilePath::*;
	
	let path = input_path.as_ref();

	if path.is_dir() {
	    Some(DirectoryPath(path))
	} else if path.is_file() {
	    if InputFilePath::is_file_extension_supported(path) {
		Some(RegularFilePath(path))
	    } else {
		None
	    }
	} else {
	    None
	}
    }

    fn is_file_extension_supported(path: &Path) -> bool {
	path.extension()
	    .map_or(false, |ext| "txt" == ext)
    }
}

pub fn generate(file_name: String, output_dir: String, language_tag: String) -> Result<()> {
    let input_path = InputFilePath::from_path(&file_name);
    
    match input_path {
	Some(InputFilePath::RegularFilePath(path)) => {
	    let site = StaticSite::from_file(path);
	    site.create(&output_dir, &language_tag)
	},
	Some(InputFilePath::DirectoryPath(path)) => {
	    let site = StaticSite::from_directory(path)?;
	    site.create(&output_dir, &language_tag)
	},
	_ => Ok(())
    }
}
