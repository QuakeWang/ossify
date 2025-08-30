use crate::error::{Error, Result};
use crate::storage::constants::CAT_CONFIRM_SIZE_THRESHOLD;
use opendal::Operator;
use std::io::IsTerminal;
use std::io::{self, Write};
use std::path::PathBuf;

pub trait FileReader {
    async fn read_and_display(&self, path: &str) -> Result<()>;
}

/// OpenDAL implementation of file reading
pub struct OpenDalFileReader {
    operator: Operator,
}

impl OpenDalFileReader {
    pub fn new(operator: Operator) -> Self {
        Self { operator }
    }

    async fn validate_cat_argument(&self, path: &str) -> Result<bool> {
        let metadata = match self.operator.stat(path).await {
            Ok(meta) => meta,
            Err(e) => {
                if e.kind() == opendal::ErrorKind::NotFound {
                    return Err(Error::PathNotFound {
                        path: PathBuf::from(path),
                    });
                } else {
                    return Err(self.map_to_cat_failed(path, e));
                }
            }
        };

        let content_length = metadata.content_length();
        if content_length > CAT_CONFIRM_SIZE_THRESHOLD {
            Ok(self.prompt_large_file_confirmation(content_length).await?)
        } else {
            Ok(true)
        }
    }

    async fn prompt_large_file_confirmation(&self, file_size: u64) -> Result<bool> {
        if !io::stdin().is_terminal() {
            println!(
                "File is large ({} MB). Skipping display in non-interactive mode.",
                file_size / (1024 * 1024)
            );
            return Ok(false);
        }

        println!(
            "File is large ({} MB). Do you want to display it? (y/N)",
            file_size / (1024 * 1024)
        );

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| Error::CatFailed {
                path: "stdin".to_string(),
                source: Box::new(e.into()),
            })?;

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => Ok(true),
            _ => {
                println!("Display cancelled.");
                Ok(false)
            }
        }
    }

    fn map_to_cat_failed(&self, path: &str, err: opendal::Error) -> Error {
        Error::CatFailed {
            path: path.to_string(),
            source: Box::new(err.into()),
        }
    }
}

impl FileReader for OpenDalFileReader {
    async fn read_and_display(&self, path: &str) -> Result<()> {
        // Check file size and get user confirmation if needed
        if !self.validate_cat_argument(path).await? {
            return Ok(());
        }

        // Read the file content
        let content = self
            .operator
            .read(path)
            .await
            .map_err(|e| Error::CatFailed {
                path: path.to_string(),
                source: Box::new(e.into()),
            })?;

        // Write to stdout
        io::stdout()
            .write_all(&content.to_vec())
            .map_err(|e| Error::CatFailed {
                path: path.to_string(),
                source: Box::new(e.into()),
            })?;

        Ok(())
    }
}
