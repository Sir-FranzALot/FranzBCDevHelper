use crate::application::{error::AppError, ports::filesystem::FilesystemRepository};
use std::path::{Path, PathBuf};
use tar::Builder;
use tokio::fs;
use zip::ZipArchive;

struct TauriFilesystemRepository {}

impl FilesystemRepository for TauriFilesystemRepository {
    async fn create_dir_all(path: impl AsRef<Path>) -> Result<(), AppError> {
        fs::create_dir_all(path)
            .await
            .map_err(|err| AppError::Repository(err.to_string()))
    }

    async fn copy_dir_recursively(
        &self,
        src: impl AsRef<Path>,
        dst: impl AsRef<Path>,
    ) -> Result<(), AppError> {
        fs::create_dir_all(&dst)
            .await
            .map_err(|err| AppError::Repository(err.to_string()))?;
        let mut src_entries = fs::read_dir(src)
            .await
            .map_err(|err| AppError::Repository(err.to_string()))?;
        while let Some(entry) = src_entries
            .next_entry()
            .await
            .map_err(|err| AppError::Repository(err.to_string()))?
        {
            let ty = entry
                .file_type()
                .await
                .map_err(|err| AppError::Repository(err.to_string()))?;
            if ty.is_dir() {
                Box::pin(
                    self.copy_dir_recursively(entry.path(), dst.as_ref().join(entry.file_name())),
                )
                .await
                .map_err(|err| AppError::Repository(err.to_string()))?;
            } else {
                fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))
                    .await
                    .map_err(|err| AppError::Repository(err.to_string()))?;
            }
        }
        Ok(())
    }

    async fn compress_dir(path: PathBuf) -> Result<Vec<u8>, AppError> {
        let tar_data = tokio::task::spawn_blocking(move || -> Result<Vec<u8>, std::io::Error> {
            let mut archive = Builder::new(Vec::new());
            archive.append_dir_all("", path)?;
            archive.finish()?;

            let tar_data = archive.into_inner()?;
            Ok(tar_data)
        })
        .await
        .map_err(|err| AppError::Repository(err.to_string()))?
        .map_err(|err| AppError::Repository(err.to_string()))?;

        Ok(tar_data)
    }

    async fn extract_zip(src: &Path, dst: &Path) -> Result<(), AppError> {
        let temp_extract_path = dst.with_extension("extracting");

        if temp_extract_path
            .try_exists()
            .map_err(|err| AppError::Repository(err.to_string()))?
        {
            fs::remove_dir_all(&temp_extract_path)
                .await
                .map_err(|err| AppError::Repository(err.to_string()))?;
        }

        let zip_path_owned = src.to_owned();
        let temp_extract_path_owned = temp_extract_path.clone();

        tokio::task::spawn_blocking(move || -> Result<(), AppError> {
            let file = std::fs::File::open(&zip_path_owned)
                .map_err(|err| AppError::Repository(err.to_string()))?;
            let mut archive =
                ZipArchive::new(file).map_err(|err| AppError::Repository(err.to_string()))?;
            archive
                .extract(&temp_extract_path_owned)
                .map_err(|err| AppError::Repository(err.to_string()))?;

            Ok(())
        })
        .await
        .map_err(|err| AppError::Repository(err.to_string()))?
        .map_err(|err| AppError::Repository(err.to_string()))?;

        fs::rename(&temp_extract_path, dst)
            .await
            .map_err(|err| AppError::Repository(err.to_string()))?;

        fs::remove_file(src)
            .await
            .map_err(|err| AppError::Repository(err.to_string()))?;

        Ok(())
    }
}
