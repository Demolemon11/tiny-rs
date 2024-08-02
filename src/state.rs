use crate::images_path::collect_images_path;
use app_theme::AppTheme;
use button_style_state::ButtonStyle;
use log_text_state::LogText;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tinify::async_bin::Tinify;
pub mod app_theme;
pub mod button_style_state;
pub mod log_text_state;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub theme: AppTheme,
    pub button_style: ButtonStyle,
}
impl Default for Config {
    fn default() -> Self {
        let theme = AppTheme::Dark;
        let button_style = ButtonStyle::Standard;
        Self {
            theme,
            button_style,
        }
    }
}

pub struct Cache {
    pub rfd_opened_path: Paths,
    pub paths: Paths,
    pub api_key: String,
    pub log_text: LogText,
}

impl Default for Cache {
    fn default() -> Self {
        let rfd_opened_path = Paths::default();
        let paths = Paths::default();
        let api_key = String::new();
        let log_text = LogText::Null;
        Self {
            rfd_opened_path,
            paths,
            api_key,
            log_text,
        }
    }
}
impl Cache {
    pub fn clear_paths(&mut self) {
        self.paths.0.clear();
        self.rfd_opened_path.0.clear()
    }
    pub fn rfd_again(&mut self) {
        let rfd_opened_path = FileDialog::new().pick_folder().unwrap_or_default();
        let paths = collect_images_path(&rfd_opened_path);
        let iter1 = self.paths.clone().0.into_iter();
        let iter2 = paths.into_iter();
        self.paths.0 = iter1.chain(iter2).collect()
    }
}

#[derive(Clone, Default)]
pub struct Paths(pub Vec<PathBuf>);

impl Iterator for Paths {
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        println!("ITEROK");
        self.0.clone().into_iter().next()
    }
}
impl Paths {
    pub fn to_display(&self) -> String {
        self.0
            .iter()
            .map(|item| {
                println!("{}sssss", item.to_string_lossy());
                item.to_str().unwrap().to_owned()
            })
            .collect::<String>()
    }
}

pub async fn process_images(paths: &mut Paths, tinify: Tinify) -> anyhow::Result<()> {
    for p in paths {
        let iter = &*p.to_string_lossy();
        println!("34rgreg54y456");

        println!("{iter}");
        let mut iter = iter.split(".");
        let front = iter.next().unwrap_or("new");
        let extention = iter.last().unwrap();

        let new_file_name = format!("{}-optimized.{}", front, extention);
        println!("34rgreg54y456");
        tinify
            .get_async_client()?
            .from_file(p.clone())
            .await?
            .to_file(new_file_name.clone())
            .await?;
    }
    Ok(())
}
