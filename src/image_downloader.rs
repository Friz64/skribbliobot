use crate::image_converter;
use gdk_pixbuf::{prelude::*, Pixbuf, PixbufLoader};
use regex::Regex;
use reqwest::blocking::Client;
use std::io::Read;

pub enum DownloadImageError {
    Error(String),
    NoImagesLeft,
}

pub struct ImageDownloader {
    client: Client,
    url_list: Vec<String>,
}

impl ImageDownloader {
    pub fn new(image_query: &str) -> Result<ImageDownloader, reqwest::Error> {
        let images_url = format!(
            "https://images.search.yahoo.com/search/images?imgty=clipart&p={}",
            image_query
        );

        let regex = Regex::new(r#"<img data-src='([^']+)' alt=''"#).unwrap();
        let client = Client::new();

        let text = client.get(&images_url).send()?.text()?;

        let mut url_list: Vec<_> = regex
            .captures_iter(&text)
            .map(|capture| capture[1].to_string())
            .collect();

        // most important ones first
        url_list.reverse();

        Ok(ImageDownloader { client, url_list })
    }

    pub fn download_image(&mut self) -> Result<Vec<u8>, DownloadImageError> {
        match self.url_list.pop() {
            Some(url) => match self.client.get(&url).send() {
                Ok(mut response) => {
                    let mut data = Vec::new();
                    response.read_to_end(&mut data).unwrap();

                    Ok(data)
                }
                Err(err) => Err(DownloadImageError::Error(err.to_string())),
            },
            None => Err(DownloadImageError::NoImagesLeft),
        }
    }
}

pub fn pixbuf_from_memory(data: &[u8]) -> Option<Pixbuf> {
    let pixbuf_loader = PixbufLoader::new();
    pixbuf_loader.write(&data).unwrap();

    let result = pixbuf_loader.get_pixbuf().map(|pixbuf| {
        let (new_width, new_height) = image_converter::resize_dimensions(
            pixbuf.get_width() as _,
            pixbuf.get_height() as _,
            150,
            150,
            false,
        );

        pixbuf
            .scale_simple(
                new_width as _,
                new_height as _,
                gdk_pixbuf::InterpType::Bilinear,
            )
            .unwrap()
    });

    pixbuf_loader.close().unwrap();

    result
}
