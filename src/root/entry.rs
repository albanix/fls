use std::fs;

// WHY FILE? In UNIX EVERYTHING IS A FILE, even your mother
pub struct RootEntity {
    pub file_name: String,
    pub file_type: String
}

impl RootEntity {
    pub fn new(path: &str) -> Self {
	let list_files = fs::read_dir(path).unwrap();

	let mut file_name = String::new();
	let mut file_type = String::new();

	for f in list_files {
	    let f = f.unwrap();
	    file_name = f.file_name().to_string_lossy().into_owned();
	    file_type = if f.file_type().unwrap().is_file() {
		String::from("file")
	    } else {
		String::from("folder")
	    };
	}
	
	Self {
	    file_name: file_name,
	    file_type: file_type
	}
    }
}
