use std::error::Error;
use std::path::PathBuf;
use crate::paths;

const DETECTION_MODEL_URL: &str = "https://ocrs-models.s3-accelerate.amazonaws.com/text-detection.rten";
const RECOGNITION_MODEL_URL: &str = "https://ocrs-models.s3-accelerate.amazonaws.com/text-recognition.rten";
pub(crate) const DETECTION_MODEL_FILE_NAME: &str = "text-detection.rten";
pub(crate) const RECOGNITION_MODEL_FILE_NAME: &str = "text-recognition.rten";

pub async fn download_rten_models() -> Result<(), Box<dyn Error>> {
    download_if_not_exists(DETECTION_MODEL_URL, DETECTION_MODEL_FILE_NAME).await?;
    download_if_not_exists(RECOGNITION_MODEL_URL, RECOGNITION_MODEL_FILE_NAME).await?;
    Ok(())
}

async fn download_if_not_exists(
    url: &str,
    file_name: &str,
) -> Result<(), Box<dyn Error>> {
    let file_path = paths::app_dir_path()?.join(file_name);
    if !file_path.exists() {
        download_file(url, file_path).await?;
    }
    Ok(())
}

async fn download_file(
    url: &str,
    path: PathBuf
) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let mut file = std::fs::File::create(path)?;
        let bytes = response.bytes().await?;
        std::io::copy(&mut bytes.as_ref(), &mut file)?;
    } else {
        response.error_for_status()?;
    }

    Ok(())
}
