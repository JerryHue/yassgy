mod static_site;
mod html_page;
use static_site::StaticSite;
use std::path::Path;
use std::io::Result;

pub fn generate(file_name: String, output_dir: String, language_tag: String) -> Result<()> {
    let input_path = Path::new(&file_name);

    if input_path.is_dir() {
        let site = StaticSite::from_directory(input_path)?;
        site.create(Path::new(&output_dir), &language_tag)?;
    } else if input_path.is_file() {
        let site = StaticSite::from_file(input_path);
        site.create(Path::new(&output_dir), &language_tag)?;
    }

    return Ok(());
}