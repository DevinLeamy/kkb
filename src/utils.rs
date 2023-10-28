use std::{path::PathBuf, env};

pub fn generate_image_path<P: Into<PathBuf> + Clone>(folder: P) -> PathBuf {
    let mut image_index = 0;
    loop {
        let path = folder
            .clone()
            .into()
            .join(format!("kkb-image-{image_index}.png"));
        if !path.exists() {
            return path;
        }
        image_index += 1;
    }
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
