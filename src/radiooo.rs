use core::panic;
use log::debug;
use reqwest::blocking::Client;
use serde::{de, Deserialize, Serialize};
use std::{collections::HashMap, io::Read, vec};
use tokio::{runtime::Handle, task};

pub const DECADES: [i32; 13] = [
    1900, 1910, 1920, 1930, 1940, 1950, 1960, 1970, 1980, 1990, 2000, 2010, 2020,
];

pub const MOODS: [&str; 3] = ["SLOW", "WEIRD", "FAST"];

pub const COUNTRY_CODES: [&str; 239] = [
    "AFG", "ALB", "DZA", "ASM", "AND", "AGO", "AIA", "ATA", "ATG", "ARG", "ARM", "ABW", "AUS",
    "AUT", "AZE", "BHS", "BHR", "BGD", "BRB", "BLR", "BEL", "BLZ", "BEN", "BMU", "BTN", "BOL",
    "BIH", "BWA", "BRA", "IOT", "BRN", "BGR", "BFA", "BDI", "CPV", "KHM", "CMR", "CAN", "CYM",
    "CAF", "TCD", "CHL", "CHN", "CXR", "CCK", "COL", "COM", "COG", "COD", "COK", "CRI", "HRV",
    "CUB", "CUW", "CYP", "CZE", "DNK", "DJI", "DMA", "DOM", "ECU", "EGY", "SLV", "GNQ", "ERI",
    "EST", "SWZ", "ETH", "FJI", "FIN", "FRA", "GUF", "PYF", "GAB", "GMB", "GEO", "DEU", "GHA",
    "GIB", "GRC", "GRL", "GRD", "GLP", "GUM", "GTM", "GGY", "GIN", "GNB", "GUY", "HTI", "HND",
    "HKG", "HUN", "ISL", "IND", "IDN", "IRN", "IRQ", "IRL", "IMN", "ISR", "ITA", "CIV", "JAM",
    "JPN", "JEY", "JOR", "KAZ", "KEN", "KIR", "PRK", "KOR", "KWT", "KGZ", "LAO", "LVA", "LBN",
    "LSO", "LBR", "LBY", "LIE", "LTU", "LUX", "MAC", "MDG", "MWI", "MYS", "MDV", "MLI", "MLT",
    "MHL", "MTQ", "MRT", "MUS", "MYT", "MEX", "FSM", "MDA", "MCO", "MNG", "MNE", "MSR", "MAR",
    "MOZ", "MMR", "NAM", "NRU", "NPL", "NLD", "NCL", "NZL", "NIC", "NER", "NGA", "NIU", "NFK",
    "MKD", "MNP", "NOR", "OMN", "PAK", "PLW", "PSE", "PAN", "PNG", "PRY", "PER", "PHL", "PCN",
    "POL", "PRT", "PRI", "QAT", "ROU", "RUS", "RWA", "REU", "BLM", "SHN", "KNA", "LCA", "MAF",
    "SPM", "VCT", "WSM", "SMR", "STP", "SAU", "SEN", "SRB", "SYC", "SLE", "SGP", "SXM", "SVK",
    "SVN", "SLB", "SOM", "ZAF", "SSD", "ESP", "LKA", "SDN", "SUR", "SJM", "SWE", "CHE", "SYR",
    "TWN", "TJK", "TZA", "THA", "TLS", "TGO", "TKL", "TON", "TTO", "TUN", "TUR", "TKM", "TCA",
    "TUV", "UGA", "UKR", "ARE", "GBR", "USA", "URY", "UZB", "VUT", "VEN", "VNM", "VGB", "VIR",
    "WLF", "ESH", "YEM", "ZMB", "ZWE",
];

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CountryForDecade {
    pub SLOW: Vec<String>,
    pub FAST: Vec<String>,
    pub WEIRD: Vec<String>,
}

impl CountryForDecade {
    pub fn to_hash_map(&self) -> HashMap<&str, Vec<String>> {
        let mut hm = HashMap::new();
        hm.insert("SLOW", self.SLOW.clone());
        hm.insert("FAST", self.FAST.clone());
        hm.insert("WEIRD", self.WEIRD.clone());
        hm
    }
}

pub fn get_country_for_decade(decade: i32) -> Option<CountryForDecade> {
    return task::block_in_place(|| {
        // Create a new reqwest client
        let client = Client::new();

        // Perform the GET request
        let response = client
            .get(format!(
                "https://radiooooo.com/country/mood?decade={}",
                decade
            ))
            .send()
            .expect("failed to send https://radiooooo.com/country/mood req");

        // Check if the request was successful
        if response.status().is_success() {
            // Deserialize the response body to ApiResponse
            let api_response: CountryForDecade = response
                .json()
                .expect("failed to unmarshall https://radiooooo.com/country/mood req");
            return Some(api_response);
        }
        None
    });
}

pub fn get_track(mood: &str, decade: i32, country: &str) -> Option<Track> {
    return task::block_in_place(|| {
        // Perform blocking operation here

        let client = Client::new();
        // Define your request payload
        let payload = ExploreRequest {
            mode: "taxi".to_string(),
            isocodes: vec![country.to_string()],
            decades: vec![decade],
            moods: vec![mood.to_string()],
        };

        let payload_json = serde_json::to_string(&payload).expect("failed to marshall");
        // Send the POST request
        let response = client
            .post("https://radiooooo.com/play")
            .json(&payload) // Serialize the payload to JSON
            .send()
            .expect("an error happened");

        // Ensure the request was successful
        if response.status().is_success() {
            // Parse the JSON response
            // debug!("rsp: {:?}", response.text().unwrap());

            let api_response: Track = response
                .json()
                .expect("should not have a problem unmarshalling json");
            debug!("req: {} {} {}=>{}", mood, decade, country, api_response._id);
            // return None;
            return Some(api_response);
        }

        panic!("{:?} {:?}", payload_json, response);
    });
}

#[derive(Serialize, Debug, PartialEq)]
struct ExploreRequest {
    mode: String,
    isocodes: Vec<String>,
    decades: Vec<i32>,
    moods: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Track {
    pub _id: String,
    pub mood: String,
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub songwriter: Option<String>, // This field seems to be always present but can be an empty string
    pub label: Option<String>,
    pub country: String,
    pub year: String,
    pub decade: i32,
    pub length: u32,
    pub uuid: String,
    pub ext: Option<Ext>,
    pub image: Option<Image>,
    pub likes: u32,
    pub profile_id: String,
    pub cover: Option<Image>,
    pub image_v: u32,
    pub liked: u32,
    pub links: Links,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Ext {
    track: String,
    cover: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Image {
    path: String,
    filename: String,
    color: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Links {
    pub mpeg: String,
    pub ogg: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_track_deserialization() {
        let json_data = r##"
        {
  "_id": "5d330a5a06fb03d8872a5d58",
  "mood": "FAST",
  "title": "Busco El Sol, No Sé Adonde Voy",
  "artist": "Caballo Vapor",
  "album": "Busco El Sol, No Sé Adonde Voy SP",
  "songwriter": "",
  "label": "MH",
  "country": "ARG",
  "year": "1975",
  "decade": 1970,
  "length": 199,
  "uuid": "380ca57f-188e-4795-9b17-f1721a7e8188",
  "ext": {
    "track": "mp3",
    "cover": "jpg"
  },
  "image": {
    "path": "cover/ARG/1970/",
    "filename": "380ca57f-188e-4795-9b17-f1721a7e8188.jpg",
    "color": "#d56f68"
  },
  "likes": 324,
  "profile_id": "5d3306de06fb03d8871fd12f",
  "cover": {
    "path": "cover/ARG/1970/",
    "filename": "380ca57f-188e-4795-9b17-f1721a7e8188.jpg",
    "color": "#d56f68"
  },
  "image_v": 0,
  "liked": 0,
  "links": {
    "mpeg": "https://radiooooo-track.b-cdn.net/ARG/1970/380ca57f-188e-4795-9b17-f1721a7e8188.mp3?token=54gfmCjcPcG77Lq_-fmh6A&expires=1723926778",
    "ogg": "https://radiooooo-track.b-cdn.net/ARG/1970/380ca57f-188e-4795-9b17-f1721a7e8188.ogg?token=hNaCodxGlb3ZCvPnrVLi7w&expires=1723926778"
  }
}"##;

        let expected_track = Track {
            _id: "5d330a5a06fb03d8872a5d58".to_string(),
            mood: "FAST".to_string(),
            title: "Busco El Sol, No Sé Adonde Voy".to_string(),
            artist: "Caballo Vapor".to_string(),
            album: Some("Busco El Sol, No Sé Adonde Voy SP".to_string()),
            songwriter: Some("".to_string()),
            label: Some("MH".to_string()),
            country: "ARG".to_string(),
            year: "1975".to_string(),
            decade: 1970,
            length: 199,
            uuid: "380ca57f-188e-4795-9b17-f1721a7e8188".to_string(),
            ext: Some(Ext {
                track: "mp3".to_string(),
                cover: Some("jpg".to_string()),
            }),
            image: Some(Image {
                path: "cover/ARG/1970/".to_string(),
                filename: "380ca57f-188e-4795-9b17-f1721a7e8188.jpg".to_string(),
                color: Some("#d56f68".to_string()),
            }),
            likes: 324,
            profile_id: "5d3306de06fb03d8871fd12f".to_string(),
            cover: Some(Image {
                path: "cover/ARG/1970/".to_string(),
                filename: "380ca57f-188e-4795-9b17-f1721a7e8188.jpg".to_string(),
                color: Some("#d56f68".to_string()),
            }),
            image_v: 0,
            liked: 0,
            links: Links {
                mpeg: "https://radiooooo-track.b-cdn.net/ARG/1970/380ca57f-188e-4795-9b17-f1721a7e8188.mp3?token=54gfmCjcPcG77Lq_-fmh6A&expires=1723926778".to_string(),
                ogg: "https://radiooooo-track.b-cdn.net/ARG/1970/380ca57f-188e-4795-9b17-f1721a7e8188.ogg?token=hNaCodxGlb3ZCvPnrVLi7w&expires=1723926778".to_string(),
            },
        };

        let track: Track = serde_json::from_str(json_data).expect("Failed to deserialize JSON");

        assert_eq!(track, expected_track);
    }
}
