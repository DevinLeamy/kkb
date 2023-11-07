use std::{env, fs, path::PathBuf};

use directories_next::ProjectDirs;
use uuid::Uuid;

pub fn generate_image_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("", "", "kkb").unwrap();
    let image_dir = proj_dirs.data_local_dir().join("images");
    fs::create_dir_all(&image_dir).ok().unwrap();
    let unique_file_name = format!("image-{}.png", Uuid::new_v4());
    let unique_path = image_dir.join(unique_file_name);

    unique_path
}

pub fn path_as_absolute_path<P: Into<PathBuf>>(path: P) -> PathBuf {
    let path: PathBuf = path.into();
    if path.is_absolute() {
        return path;
    }

    let current_dir = env::current_dir().unwrap();
    let absolute_path = current_dir.join(path);
    let canonical_path = absolute_path.canonicalize().unwrap();
    canonical_path
}
