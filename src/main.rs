use std::{collections::HashMap, thread::sleep, time::Duration};

use macroquad::{audio::*, prelude::*};

pub const BG_COLOR: Color = Color::new(25f32 / 255f32, 25f32 / 255f32, 25f32 / 255f32, 255f32);

fn conf() -> Conf {
    Conf {
        window_title: "Klassisk musikkhistorie pugging".to_string(),
        fullscreen: false,
        window_width: 800,
        window_height: 800,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(conf())]
async fn main() {
    sleep(Duration::from_secs_f32(2f32));
    let mut music = MusicManager::new().await;
    music.play_song();

    loop {
        clear_background(BG_COLOR);

        next_frame().await;
    }
}

pub struct MusicLoader(HashMap<Song, Sound>);

impl MusicLoader {
    pub async fn new() -> Self {
        let mut map = HashMap::new();

        for i in 0..8 {
            map.insert(
                Song::from(i),
                match load_sound(format!("./files/{}", Song::from(i).file_name()).as_str()).await {
                    Ok(sound) => sound,
                    Err(er) => panic!(
                        "Couldn't load sound {}, err message: \n{}",
                        Song::from(i).file_name(),
                        er.to_string()
                    ),
                },
            );
        }

        Self(map)
    }
}

pub struct MusicManager {
    current_song: Song,
    loader: MusicLoader,
}

impl MusicManager {
    async fn new() -> Self {
        Self {
            current_song: Song::random(),
            loader: MusicLoader::new().await,
        }
    }

    fn play_song(&self) {
        play_sound_once(self.loader.0.get(&self.current_song).unwrap())
    }

    fn stop_song(&self) {
        stop_sound(self.loader.0.get(&self.current_song).unwrap())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Song {
    OVirdissimaVirga,
    Lamentation1,
    TheKingOfDenmarksGalliard,
    Halleluja,
    Fløytekvartett,
    SymfoniNr5,
    GjendinesBådnlåt,
    PierotLunaire,
    Epitaffio,
}

impl Song {
    fn names(&self) -> Vec<String> {
        match self {
            Self::OVirdissimaVirga => vec!["O viridissima virga"],
            Self::Lamentation1 => vec!["Lamentation 1"],
            Self::TheKingOfDenmarksGalliard => vec![
                "The king of denmarks galliard",
                "The king of denmark's galliard",
            ],
            Self::Halleluja => vec!["Halleluja-koret fra Messias"],
            Self::Fløytekvartett => vec!["Fløytekvartett i A-dur", "Fløytekvartett"],
            Self::SymfoniNr5 => vec![
                "Symfoni nr 5",
                "Symfoni nr 5 \"skjebnesymfonien\"",
                "\"skjebnesymfonien\"",
            ],
            Self::GjendinesBådnlåt => vec!["Gjendines bådnlåt"],
            Self::PierotLunaire => vec!["Pierot lunaire", "Den månesyke Pierot"],
            Self::Epitaffio => vec!["Epitaffio", "Epitaffio for orkester og lydbånd"],
        }
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    fn file_name(&self) -> String {
        format!(
            "{}.mp3",
            match self {
                Self::OVirdissimaVirga => "o_virdissima_virga",
                Self::Lamentation1 => "lamentation_1",
                Self::TheKingOfDenmarksGalliard => "the_king_of_denmarks_galliard",
                Self::Halleluja => "halleluja",
                Self::Fløytekvartett => "fløytekvartett_i_a_dur",
                Self::SymfoniNr5 => "symfoni_nr_5",
                Self::GjendinesBådnlåt => "gjendines_bådnlåt",
                Self::PierotLunaire => "pierot_lunaire",
                Self::Epitaffio => "epitaffio",
            }
        )
    }

    fn artist(&self) -> String {
        match self {
            Self::OVirdissimaVirga => "Hildegard von Bingen",
            Self::Lamentation1 => "Giovanni da Palestrina",
            Self::TheKingOfDenmarksGalliard => "John Dowland",
            Self::Halleluja => "Georg Friedrich Handel",
            Self::Fløytekvartett => "Wolfgang Amadeus Mozart",
            Self::SymfoniNr5 => "Ludwig van Beethoven",
            Self::GjendinesBådnlåt => "Edvard Grieg",
            Self::PierotLunaire => "Arnold Schonberg",
            Self::Epitaffio => "Arne Nordheim",
        }
        .to_string()
    }

    fn period(&self) -> String {
        match self {
            Self::OVirdissimaVirga => "Middelalderen",
            Self::Lamentation1 => "Renessansen",
            Self::TheKingOfDenmarksGalliard => "Renessansen",
            Self::Halleluja => "Barokken",
            Self::Fløytekvartett => "Wienerklassisismen",
            Self::SymfoniNr5 => "Wienerklassisismen",
            Self::GjendinesBådnlåt => "Romantikken",
            Self::PierotLunaire => "Moderne musikk",
            Self::Epitaffio => "Moderne musikk",
        }
        .to_string()
    }

    fn random() -> Song {
        return Self::from(rand::gen_range(0u8, 8));
    }
}

impl From<u8> for Song {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::OVirdissimaVirga,
            1 => Self::Lamentation1,
            2 => Self::TheKingOfDenmarksGalliard,
            3 => Self::Halleluja,
            4 => Self::Fløytekvartett,
            5 => Self::SymfoniNr5,
            6 => Self::GjendinesBådnlåt,
            7 => Self::PierotLunaire,
            8 => Self::Epitaffio,
            _ => panic!("Could not get song from u8"),
        }
    }
}
