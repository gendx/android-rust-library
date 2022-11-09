use std::collections::HashSet;
use std::fs::read_dir;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    visit(
        "".into(),
        "/home/dev/rustc-build/rust/".into(),
        &mut HashSet::new(),
    )
}

fn visit(output_name: String, path: PathBuf, inodes: &mut HashSet<u64>) -> io::Result<()> {
    for entry in read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let inode = metadata.ino();

        if !inodes.insert(inode) {
            eprintln!("Skipping hard link (inode = {}): {:?}", inode, entry.path());
            continue;
        }

        // TODO: sanitize file name.
        let file_name = entry.file_name().into_string().unwrap();
        let item_name = if output_name.is_empty() {
            file_name
        } else {
            output_name.clone() + ";" + &file_name
        };

        if metadata.is_dir() {
            visit(item_name, entry.path(), inodes)?;
        } else if metadata.is_file() {
            println!("{} {}", item_name, metadata.len());
        }
    }
    Ok(())
}
