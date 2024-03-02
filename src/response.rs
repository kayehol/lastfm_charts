use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Response{
    pub topartists: Topartists,
}

#[derive(Serialize, Deserialize)]
pub struct Topartists {
    pub artist: Vec<Artist>,
    #[serde(rename = "@attr")]
    pub attr: TopartistsAttr,
}

#[derive(Serialize, Deserialize)]
pub struct Artist {
    streamable: String,
    image: Vec<Image>,
    mbid: String,
    url: String,
    playcount: String,
    #[serde(rename = "@attr")]
    pub attr: ArtistAttr,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ArtistAttr {
    pub rank: String,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    size: String,
    #[serde(rename = "#text")]
    text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopartistsAttr {
    pub user: String,
    total_pages: String,
    page: String,
    total: String,
    per_page: String,
}
