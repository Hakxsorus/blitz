use fuzzy_matcher::FuzzyMatcher;
use std::error::Error;
use std::path::PathBuf;
use fuzzy_matcher::skim::SkimMatcherV2;
use ocrs::{OcrEngine, OcrEngineParams};
use rten::Model;
use rten_imageio::read_image;
use rten_tensor::AsView;
use xcap::{Window, XCapError};
use crate::blacklist::Blacklist;
use crate::paths;
use image::io::Reader as ImageReader;

#[derive(Debug)]
pub(crate) struct DetectorMatchInfo {
    pub username: String,
    pub score: i64
}

const RISK_APPLICATION_TITLE: &str = "RISK";

pub fn detect() -> Result<Vec<DetectorMatchInfo>, Box<dyn Error>> {
    // First let's take the screenshot, crop it, and save it.
    let windows = windows()?;
    let risk_application_window = risk_application_window(&windows).ok_or("Unable to find Risk window.")?;
    let risk_scrshot_path = scrshot_window(&risk_application_window)?;
    crop_and_save_player_cards(&risk_scrshot_path)?;
    // Now create the OCR engine and get the text from the player card screenshots.
    let engine = create_ocr_engine()?;
    let mut detections: Vec<String> = Vec::new();
    for i in 0..6 {
        let player_scrshot_path = paths::player_scrshot_path(&i)?;
        let lines = detect_text_from_scrshot(&engine, &player_scrshot_path)?;
        detections.extend(lines);
    }
    // Then we need to load the blacklist.
    let blacklist_path = paths::blacklist_path()?;
    let blacklist = Blacklist::load(&blacklist_path)?;
    // Compare the morons and countries in a fuzzy match.
    let mut matches: Vec<DetectorMatchInfo> = Vec::new();
    let matcher = SkimMatcherV2::default();
    for line in detections
        .iter()
        .filter(|l| l.len() > 1)
    {
        for moron in blacklist.morons.iter() {
            match matcher.fuzzy_match(&line, &moron.username.to_lowercase()) {
                None => { continue; }
                Some(score) => {
                    matches.push(DetectorMatchInfo {
                        username: moron.username.to_string(),
                        score: score
                    })
                }
            }
        }
    }

    Ok(matches)
}

fn windows() -> Result<Vec<Window>, XCapError> {
    Window::all()
}

fn risk_application_window(windows: &[Window]) -> Option<Window> {
    windows
        .iter()
        .find(|w| w.title() == RISK_APPLICATION_TITLE)
        .cloned()
}

fn scrshot_window(window: &Window) -> Result<PathBuf, Box<dyn Error>> {
    // Capture the given window and save it to the screenshot path.
    let image = window.capture_image()?;
    let scrshot_path = paths::scrshot_path()?;
    image.save(&scrshot_path)?;
    Ok(scrshot_path)
}

// TODO: Optimise this method to account for screen sizes other than 1920x1080 px.
fn crop_and_save_player_cards(scrshot_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    // Crop the surrounding space out of the player list.
    let mut image = ImageReader::open(&scrshot_path)?.decode()?;
    let player_list_width = 1200;
    let player_list_height = 550;
    let player_list_start_x = (image.width() - player_list_width) / 2;
    let player_list_start_y = (image.height() - player_list_height) / 2;
    let player_list_image = image.crop(
        player_list_start_x,
        player_list_start_y,
        player_list_width,
        player_list_height
    );
    // Crop the individual players cards out of the player list.
    let player_card_width = 600;
    let player_card_height = 180;
    for row in 0..3 {
        for col in 0..2 {
            let player_card_start_x = col * player_card_width;
            let player_card_start_y = row * player_card_height;
            let player_card_image = player_list_image.clone().crop(
                player_card_start_x,
                player_card_start_y,
                player_card_width,
                player_card_height);
            let player_card_index = row * 2 + col;
            let player_scrshot_path = paths::player_scrshot_path(&player_card_index)?;
            player_card_image.save(player_scrshot_path)?;
        }
    }

    Ok(())
}

fn create_ocr_engine() -> Result<OcrEngine, Box<dyn Error>> {
    // Get the models from the paths and load them.
    let detection_model_path = paths::detection_model_path()?;
    let detection_model_data = std::fs::read(detection_model_path)?;
    let recognition_model_path = paths::recognition_model_path()?;
    let recognition_model_data = std::fs::read(recognition_model_path)?;
    let detection_model = Model::load(&detection_model_data)?;
    let recognition_model = Model::load(&recognition_model_data)?;
    // Create an OCR engine given the models.
    Ok(OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        debug: false,
        decode_method: Default::default(),
    })?)
}

fn detect_text_from_scrshot(
    ocr_engine: &OcrEngine,
    scrshot_path: &PathBuf
)  -> Result<Vec<String>, Box<dyn Error>> {
    // Detect the text from the image.
    let image = read_image(&scrshot_path.display().to_string())?;
    let ocr_input = ocr_engine.prepare_input(image.view())?;
    let text = ocr_engine.get_text(&ocr_input)?;
    // Split it on newlines to get an array of detected text chunks.
    Ok(text.split('\n')
        .map(|s| s.to_string().to_lowercase())
        .collect()
    )
}