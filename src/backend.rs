use crate::fetch;
use crate::gui;
use crate::response;
use clap::{load_yaml, App};

pub fn main() {
    let yaml = load_yaml!("config/cli.yaml");
    let matches = App::from(yaml).get_matches();

    if matches.is_present("num") {
        let dexnum = fetch::num_query::Variables {
            num: matches.value_of("num").unwrap().parse::<i64>().unwrap(),
        };
        let response = fetch::fetch_dex_num(dexnum).expect("Query unsuccessful!");
        response::cli_show_numresult(response);
    } else if matches.is_present("name") {
        let dexname = fetch::name_query::Variables {
            pokemon: String::from(matches.value_of("name").unwrap()),
        };
        let response = fetch::fetch_dex_name(dexname).expect("Query unsuccessful!");
        response::cli_show_nameresult(response);
    } else {
        gui::main();
    }
}
