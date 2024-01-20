use crate::imap::monitor_postbox;
use log::{error, info, LevelFilter};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::fs;
use std::fs::File;

mod fireplan;
mod imap;
mod parser;

#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
pub struct Standort {
    standort: String,
    imap_server: String,
    imap_port: u16,
    imap_user: String,
    imap_password: String,
    fireplan_api_key: String,
}

#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
pub struct Ric {
    text: String,
    ric: String,
    subric: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Configuration {
    regex_einsatzstichwort: String,
    regex_strasse: String,
    regex_ort: String,
    regex_hausnummer: String,
    regex_ortsteil: String,
    regex_einsatznrleitstelle: String,
    regex_koordinaten: String,
    regex_zusatzinfo: String,
    regex_objektname: String,
    rics: Vec<Ric>,
    standorte: Vec<Standort>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ParsedData {
    rics: Vec<Ric>,
    einsatznrlst: String,
    strasse: String,
    hausnummer: String,
    ort: String,
    ortsteil: String,
    objektname: String,
    koordinaten: String,
    einsatzstichwort: String,
    zusatzinfo: String,
}

fn main() {
    let file = if cfg!(windows) {
        format!(
            "{}\\fireplan_alarm_imap.conf",
            std::env::var("USERPROFILE").unwrap()
        )
    } else {
        "~/fireplan_alarm_imap.conf".to_string()
    };
    let content = fs::read_to_string(file).expect("Config file missing!");
    let configuration: Configuration = toml::from_str(content.as_str()).unwrap();

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create("fireplan_alarm_imap.log").unwrap(),
        ),
    ])
    .unwrap();

    let mut configuration_output = format!("Configuration: {:?}", configuration);

    for standort in configuration.standorte.clone() {
        configuration_output = configuration_output.replace(&standort.imap_password, "****");
    }

    info!("Configuration: {}", configuration_output);

    for standort in &configuration.standorte {
        match monitor_postbox(standort.clone(), configuration.clone()) {
            Ok(_) => {
                info!("monitor done: {}", standort.standort)
            }
            Err(e) => {
                error!("monitor failed: {}, {}", standort.standort, e)
            }
        };
    }
}
