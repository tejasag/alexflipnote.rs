// =======================Imports===============================

use bytes::Bytes;
use reqwest;
use reqwest::{header::AUTHORIZATION, Client, Error};
use serde::Deserialize;
use std::{borrow::Cow, collections::HashMap};

// ======================API response structs====================

#[derive(Deserialize, Debug)]
pub struct IndexAPIResponse {
    pub support_server: String,
    pub endpoints: Vec<String>,
    pub wrappers: HashMap<String, WrapperInfo>,
}

#[derive(Deserialize, Debug)]
pub struct WrapperInfo {
    pub author: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct ColorAPIResponse {
    pub blackorwhite_text: String,
    pub brightness: i32,
    pub hex: String,
    pub image: String,
    pub image_gradient: String,
    pub int: i64,
    pub name: String,
    pub rgb: String,
    pub rgb_values: HashMap<String, i32>,
    pub shade: Vec<String>,
    pub tint: Vec<String>,
}

// Add the `.as_cow` method to the Bytes struct.

pub trait CowValue {
    fn as_cow(&self) -> Cow<[u8]>;
}

impl CowValue for Bytes {
    fn as_cow(&self) -> Cow<[u8]> {
        Cow::from((&*self) as &[u8])
    }
}

// =====================Enums for image endpoints.=======================================
#[derive(Debug)]
pub enum MinecraftIcons {
    NONE = 0,
    GRASS_BLOCK = 1,
    DIAMOND = 2,
    DIAMOND_SWORD = 3,
    CREEPER = 4,
    PIG = 5,
    TNT = 6,
    COOKIE = 7,
    HEART = 8,
    BED = 9,
    CAKE = 10,
    SIGN = 11,
    RAIL = 12,
    CRAFTING_BENCH = 13,
    REDSTONE = 14,
    FIRE = 15,
    COBWEB = 16,
    CHEST = 17,
    FURNACE = 18,
    BOOK = 19,
    STONE_BLOCK = 20,
    WOODEN_PLANK_BLOCK = 21,
    IRON_INGOT = 22,
    GOLD_INGOT = 23,
    WOODEN_DOOR = 24,
    IRON_DOOR = 25,
    DIAMOND_CHESTPLATE = 26,
    FLINT_AND_STEEL = 27,
    GLASS_BOTTLE = 28,
    SPLASH_POTION = 29,
    CREEPER_SPAWNEGG = 30,
    COAL = 31,
    IRON_SWORD = 32,
    BOW = 33,
    ARROW = 34,
    IRON_CHESTPLATE = 35,
    BUCKET = 36,
    BUCKET_WITH_WATER = 37,
    BUCKET_WITH_LAVA = 38,
    BUCKET_WITH_MILK = 39,
    DIAMOND_BOOTS = 40,
    WOODEN_HOE = 41,
    BREAD = 42,
    WOODEN_SWORD = 43,
    BONE = 44,
    OAK_LOG = 45,
}

// ========================== UTILITIES =========================================
pub fn parse_filter(text: &mut str) -> &str {
    let uppercase = text.to_ascii_uppercase();
    let data = match &*uppercase {
        "BLUR" => "blur",
        "INVERT" => "invert",
        "BLACK_AND_WHITE" => "b&w",
        "DEEPFRY" => "deepfry",
        "SEPIA" => "sepia",
        "PIXELATE" => "pixelate",
        "MAGIK" => "magik",
        "JPEGIFY" => "jpegify",
        "WIDE" => "wide",
        "SNOW" => "snow",
        "GAY" => "gay",
        "COMMUNIST" => "communist",
        _ => "magik",
    };
    data
}

pub fn parse_icon(text: &mut str) -> MinecraftIcons {
    let uppercase = text.to_ascii_uppercase();
    let data = match &*uppercase {
        "GRASS_BLOCK" => MinecraftIcons::GRASS_BLOCK,
        "DIAMOND" => MinecraftIcons::DIAMOND,
        "DIAMOND_SWORD" => MinecraftIcons::DIAMOND_SWORD,
        "CREEPER" => MinecraftIcons::CREEPER,
        "PIG" => MinecraftIcons::PIG,
        "TNT" => MinecraftIcons::TNT,
        "COOKIE" => MinecraftIcons::COOKIE,
        "HEART" => MinecraftIcons::HEART,
        "BED" => MinecraftIcons::BED,
        "CAKE" => MinecraftIcons::CAKE,
        "SIGN" => MinecraftIcons::SIGN,
        "RAIL" => MinecraftIcons::RAIL,
        "CRAFTING_BENCH" => MinecraftIcons::CRAFTING_BENCH,
        "REDSTONE" => MinecraftIcons::REDSTONE,
        "FIRE" => MinecraftIcons::FIRE,
        "COBWEB" => MinecraftIcons::COBWEB,
        "CHEST" => MinecraftIcons::CHEST,
        "FURNACE" => MinecraftIcons::FURNACE,
        "BOOK" => MinecraftIcons::BOOK,
        "STONE_BLOCK" => MinecraftIcons::STONE_BLOCK,
        "WOODEN_PLANK_BLOCK" => MinecraftIcons::WOODEN_PLANK_BLOCK,
        "IRON_INGOT" => MinecraftIcons::IRON_INGOT,
        "GOLD_INGOT" => MinecraftIcons::GOLD_INGOT,
        "WOODEN_DOOR" => MinecraftIcons::WOODEN_DOOR,
        "IRON_DOOR" => MinecraftIcons::IRON_DOOR,
        "DIAMOND_CHESTPLATE" => MinecraftIcons::DIAMOND_CHESTPLATE,
        "FLINT_AND_STEEL" => MinecraftIcons::FLINT_AND_STEEL,
        "GLASS_BOTTLE" => MinecraftIcons::GLASS_BOTTLE,
        "SPLASH_POTION" => MinecraftIcons::SPLASH_POTION,
        "CREEPER_SPAWNEGG" => MinecraftIcons::CREEPER_SPAWNEGG,
        "COAL" => MinecraftIcons::COAL,
        "IRON_SWORD" => MinecraftIcons::IRON_SWORD,
        "BOW" => MinecraftIcons::BOW,
        "ARROW" => MinecraftIcons::ARROW,
        "IRON_CHESTPLATE" => MinecraftIcons::IRON_CHESTPLATE,
        "BUCKET" => MinecraftIcons::BUCKET,
        "BUCKET_WTH_WATER" => MinecraftIcons::BUCKET_WITH_WATER,
        "BUCKET_WITH_LAVA" => MinecraftIcons::BUCKET_WITH_LAVA,
        "BUCKER_WITH_MILK" => MinecraftIcons::BUCKET_WITH_MILK,
        "DIAMOND_BOOTS" => MinecraftIcons::DIAMOND_BOOTS,
        "WOODEN_HOE" => MinecraftIcons::WOODEN_HOE,
        "BREAD" => MinecraftIcons::BREAD,
        "WOODEN_SWORD" => MinecraftIcons::WOODEN_SWORD,
        "BONE" => MinecraftIcons::BONE,
        "OAK_LOG" => MinecraftIcons::OAK_LOG,
        _ => MinecraftIcons::NONE,
    };
    data
}
//================================ AlexClient struct=================================

pub struct AlexClient {
    key: &'static str,
    client: Client,
}

impl AlexClient {
    // Make a new instance of the client
    pub fn new(key: &'static str) -> AlexClient {
        let client = Client::new();
        AlexClient { key, client }
    }

    // Returns the index page JSON.
    pub fn index(&self) -> IndexAPIResponse {
        let data = reqwest::blocking::get("https://api.alexflipnote.dev/")
            .expect("Could not fetch data")
            .json::<IndexAPIResponse>()
            .expect("Could not parse JSON");
        data
    }

    // Color Endpoint
    pub async fn color(&self, hex: &str) -> Result<ColorAPIResponse, Error> {
        let mut url = String::from("https://api.alexflipnote.dev/color/");
        url.push_str(&hex);
        let req = self
            .client
            .get(&url)
            .header(AUTHORIZATION, self.key)
            .send()
            .await?;
        let data = req
            .json::<ColorAPIResponse>()
            .await
            .expect("Could not parse JSON");
        Ok(data)
    }

    // ============================IMAGE ENDPOINTS=================================
    async fn fetch_image(&self, endpoint: &str, parameters: &str) -> Result<Bytes, Error> {
        let mut base = String::from("https://api.alexflipnote.dev/");
        base.push_str(&endpoint);
        base.push_str(&parameters);
        let req = self
            .client
            .get(&base)
            .header(AUTHORIZATION, self.key)
            .send()
            .await?;
        let data = req.bytes().await.expect("Could not parse Bytes");
        Ok(data)
    }

    pub async fn drake(&self, top: &str, bottom: &str) -> Result<Bytes, Error> {
        let data = self
            .fetch_image("drake", &*format!("?top={0}&bottom={1}", &top, &bottom))
            .await;
        data
    }

    pub async fn achievement(&self, text: &str, icon: &i32) -> Result<Bytes, Error> {
        let icon_parsed = match &*icon {
            0 => String::from(""),
            _ => format!("&icon={}", icon),
        };
        let data = self
            .fetch_image("achievement", &*format!("?text={0}{1}", text, icon_parsed))
            .await;
        data
    }

    pub async fn amiajoke(&self, url: &str) -> Result<Bytes, Error> {
        let data = self
            .fetch_image("amiajoke", &*format!("?image={0}", url))
            .await;
        data
    }

    pub async fn bad(&self, url: &str) -> Result<Bytes, Error> {
        let data = self.fetch_image("bad", &*format!("?image={0}", url)).await;
        data
    }

    pub async fn birb(&self) -> Result<Bytes, Error> {
        let data = self.fetch_image("birb", "").await;
        data
    }

    pub async fn calling(&self, text: &str) -> Result<Bytes, Error> {
        let data = self
            .fetch_image("calling", &format!("?text={0}", text))
            .await;
        data
    }

    pub async fn captcha(&self, text: &str) -> Result<Bytes, Error> {
        let data = self
            .fetch_image("captcha", &format!("?text={0}", text))
            .await;
        data
    }

    pub async fn cats(&self) -> Result<Bytes, Error> {
        let data = self.fetch_image("cats", "").await;
        data
    }

    pub async fn colorify(&self, url: &str, color: &str, background: &str) -> Result<Bytes, Error> {
        let color_parsed = match &*color.to_ascii_lowercase() {
            "none" => "".to_string(),
            "null" => "".to_string(),
            "0" => "".to_string(),
            "false" => "".to_string(),
            _ => format!("&c={}", color),
        };
        let background_parsed = match &*background.to_ascii_lowercase() {
            "none" => "".to_string(),
            "null" => "".to_string(),
            "0" => "".to_string(),
            "false" => "".to_string(),
            _ => format!("&b={}", background),
        };

        let data = self
            .fetch_image(
                "colourify",
                &format!("?image={0}{1}{2}", url, color_parsed, background_parsed),
            )
            .await;
        data
    }

    pub async fn didyoumean(&self, top: &str, bottom: &str) -> Result<Bytes, Error> {
        let data = self
            .fetch_image(
                "didyoumean",
                &*format!("?top={0}&bottom={1}", &top, &bottom),
            )
            .await;
        data
    }

    pub async fn dogs(&self) -> Result<Bytes, Error> {
        let data = self.fetch_image("dogs", "").await;
        data
    }

    pub async fn facts(&self, text: &str) -> Result<Bytes, Error> {
        let data = self.fetch_image("facts", &format!("?text={0}", text)).await;
        data
    }

    pub async fn filter(&self, name: &str, text: &str) -> Result<Bytes, Error> {
        let name_parsed = name.to_string();
        let data = self
            .fetch_image("filter", &*format!("/{0}?image={1}", name_parsed, text))
            .await;
        data
    }

    pub async fn floor(&self, img: &str, text: &str) -> Result<Bytes, Error> {
        let data = self
            .fetch_image("floor", &format!("?image={0}&text={1}", &img, &text))
            .await;
        data
    }

    pub async fn fml(&self) -> Result<Bytes, Error> {
        let data = self.fetch_image("fml", "").await;
        data
    }

    pub async fn jokeoverhead(&self, url: &str) -> Result<Bytes, Error> {
        let data = self
            .fetch_image("jokeoverhead", &*format!("?image={0}", url))
            .await;
        data
    }

    pub async fn pornhub(&self, text: &str, text2: &str) -> Result<Bytes, Error> {
        let data = self
            .fetch_image("pornhub", &*format!("?text={0}&text2={1}", &text, &text2))
            .await;
        data
    }

    pub async fn sadcat(&self) -> Result<Bytes, Error> {
        let data = self.fetch_image("sadcat", "").await;
        data
    }

    pub async fn salty(&self, url: &str) -> Result<Bytes, Error> {
        let data = self
            .fetch_image("salty", &*format!("?image={0}", url))
            .await;
        data
    }

    pub async fn scroll(&self, text: &str) -> Result<Bytes, Error> {
        let data = self
            .fetch_image("scroll", &*format!("?text={0}", text))
            .await;
        data
    }

    pub async fn shame(&self, url: &str) -> Result<Bytes, Error> {
        let data = self
            .fetch_image("shame", &*format!("?image={0}", url))
            .await;
        data
    }

    pub async fn ship(&self, user: &str, user2: &str) -> Result<Bytes, Error> {
        let data = self
            .fetch_image("ship", &*format!("?user={0}&user2={1}", &user, &user2))
            .await;
        data
    }

    pub async fn supreme(&self, text: &str, mode: &str) -> Result<Bytes, Error> {
        let mode_parsed = match &*mode.to_ascii_lowercase() {
            "light" => "&light=true",
            "dark" => "&dark=true",
            _ => "",
        };
        let data = self
            .fetch_image("supreme", &*format!("?text={0}{1}", &text, &mode_parsed))
            .await;
        data
    }

    pub async fn trash(&self, face: &str, trash: &str) -> Result<Bytes, Error> {
        let data = self
            .fetch_image("trash", &*format!("?face={0}&trash={1}", &face, &trash))
            .await;
        data
    }

    pub async fn what(&self, image: &str) -> Result<Bytes, Error> {
        let data = self
            .fetch_image("what", &*format!("?image={0}", &image))
            .await;
        data
    }
}
