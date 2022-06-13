
use std::fs;
    
pub struct SiteFolder {
    pub site_folder: String,
    pub index_file: String,
}

impl SiteFolder {
    pub fn new(sitename: &str) -> SiteFolder {
        let mut site_folder = "sites/".to_owned();
        site_folder.push_str(sitename);
    
        fs::create_dir_all(&site_folder).expect("Unable to create file directory.");
    
        let mut index_file = site_folder.clone();
        index_file.push_str("/index.html");
    
        SiteFolder { site_folder, index_file }
    }
}