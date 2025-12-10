use s3_simple::*;
use std::path::Path;
use mime::Mime;
use uuid::Uuid;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageUploadError {
    #[error("Image file cannot be empty")]
    EmptyFile,
    
    #[error("Invalid image format. Only .jpg, .jpeg, .png, .gif are allowed")]
    InvalidFormat,
    
    #[error("Failed to determine content type")]
    ContentTypeError,
    
    #[error("S3 error: {0}")]
    S3Error(#[from] S3Error),
}

/// Uploads raw binary data to S3
pub async fn upload_image_async(file_name: &str, content: &[u8]) -> Result<(), S3Error> {
    dotenvy::dotenv().ok().unwrap();

    let bucket = Bucket::try_from_env().expect("env vars to be set in .env");
    bucket.put(file_name, content).await?;
    Ok(())
}

/// Validates and uploads an image file to S3
/// 
/// # Arguments
/// * `file_content` - The binary content of the file
/// * `file_name` - The original filename (used for extension detection)
/// * `folder` - The folder path within the S3 bucket where the file should be stored
/// 
/// # Returns
/// Returns the full S3 key/path where the file was uploaded if successful
pub async fn upload_image_file(
    file_content: Vec<u8>,
    file_name: &str,
    folder: &str,
) -> Result<String, ImageUploadError> {
    // Check if file is empty
    if file_content.is_empty() {
        return Err(ImageUploadError::EmptyFile);
    }

    // Validate file extension
    let allowed_extensions = ["jpg", "jpeg", "png", "gif"];
    let extension = Path::new(file_name)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .ok_or(ImageUploadError::InvalidFormat)?;

    if !allowed_extensions.contains(&extension.as_str()) {
        return Err(ImageUploadError::InvalidFormat);
    }

    // Determine content type
    let mime_type = mime_guess::from_path(file_name)
        .first()
        .ok_or(ImageUploadError::ContentTypeError)?;

    // Generate unique filename
    let unique_filename = format!(
        "Webapp/{}/{}{}",
        folder,
        Uuid::new_v4(),
        Path::new(file_name)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| format!(".{}", ext))
            .unwrap_or_default()
    );

    // Upload to S3
    upload_image_async(&unique_filename, &file_content).await?;
    
    Ok(unique_filename)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[tokio::test]
    async fn test_upload_image_validation() {
        // Test empty file
        let result = upload_image_file(vec![], "test.jpg", "test").await;
        assert!(matches!(result, Err(ImageUploadError::EmptyFile)));

        // Test invalid extension
        let result = upload_image_file(vec![1, 2, 3], "test.txt", "test").await;
        assert!(matches!(result, Err(ImageUploadError::InvalidFormat)));

        // Test valid file (won't actually upload in test)
        let test_image = fs::read("test.jpg").ok();
        if let Some(content) = test_image {
            let result = upload_image_file(content, "test.jpg", "test").await;
            // This will fail with S3Error if AWS credentials aren't set up
            // but we can't test the success case without a test S3 bucket
            println!("Test upload result: {:?}", result);
        }
    }
}