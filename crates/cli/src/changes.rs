use std::{
    collections::BTreeMap,
    fs,
    hash::{DefaultHasher, Hash, Hasher},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context as _, Result, ensure};
use tempfile::NamedTempFile;

// Keep the lock outside the user's tree so an invalid/conflicting install
// leaves no lock file or empty module directory behind. The OS releases the
// lock if the installing process exits.
pub fn lock(to: &Path) -> Result<fs::File> {
    let absolute = std::path::absolute(to)?;
    let mut key = DefaultHasher::new();
    for ancestor in absolute.ancestors() {
        match ancestor.canonicalize() {
            Ok(existing) => {
                existing
                    .join(absolute.strip_prefix(ancestor)?)
                    .hash(&mut key);
                break;
            }
            Err(error) if error.kind() == ErrorKind::NotFound => continue,
            Err(error) => return Err(error.into()),
        }
    }
    let path = std::env::temp_dir().join(format!("gpui-deck-install-{:016x}.lock", key.finish()));
    let file = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(path)?;
    file.lock()
        .with_context(|| format!("lock installation {}", to.display()))?;
    Ok(file)
}

// Stage in each destination directory so replacement uses an atomic rename
// on the same filesystem. Only generated files are ever replaced.
pub fn write(to: &Path, files: BTreeMap<PathBuf, Vec<u8>>) -> Result<Vec<PathBuf>> {
    let mut staged = Vec::new();
    for (path, bytes) in files {
        let destination = to.join(&path);
        let parent = destination.parent().unwrap();
        fs::create_dir_all(parent)?;
        let mut builder = tempfile::Builder::new();
        builder.prefix(".gpui-deck-");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            builder.permissions(fs::Permissions::from_mode(0o666));
        }
        let mut updated = builder.tempfile_in(parent)?;
        updated.write_all(&bytes)?;
        let original = match fs::metadata(&destination) {
            Ok(metadata) => {
                ensure!(
                    !metadata.permissions().readonly(),
                    "{} is read-only",
                    destination.display()
                );
                let backup = NamedTempFile::new_in(parent)?;
                fs::copy(&destination, backup.path())?;
                updated.as_file().set_permissions(metadata.permissions())?;
                Some(backup)
            }
            Err(error) if error.kind() == ErrorKind::NotFound => None,
            Err(error) => return Err(error.into()),
        };
        staged.push((path, updated, original));
    }
    // Interrupted installs can be retried against the old metadata until the
    // new component and block declarations are in place.
    staged.sort_by_key(|(path, _, _)| {
        path == Path::new(super::MANIFEST) || path == Path::new(super::block::INDEX)
    });
    let mut applied: Vec<(PathBuf, Option<NamedTempFile>)> = Vec::new();
    for (path, updated, original) in staged {
        let destination = to.join(&path);
        let change = if original.is_some() {
            updated.persist(&destination)
        } else {
            updated.persist_noclobber(&destination)
        };
        if let Err(error) = change {
            for (relative, original) in applied.into_iter().rev() {
                let path = to.join(relative);
                if let Some(backup) = original {
                    backup
                        .persist(&path)
                        .with_context(|| format!("restore {}", path.display()))?;
                } else {
                    fs::remove_file(&path)
                        .with_context(|| format!("remove incomplete {}", path.display()))?;
                }
            }
            return Err(error).with_context(|| format!("install {}", destination.display()));
        }
        applied.push((path, original));
    }
    Ok(applied.into_iter().map(|(path, _)| path).collect())
}
