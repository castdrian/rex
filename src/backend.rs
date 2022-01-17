use graphql_client::{GraphQLQuery, Response};
use std::error::Error;
use clap::{App, load_yaml};
use comfy_table::Table;
use reqwest;
use tokio;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.graphql",
    query_path = "src/gql/num_query.graphql",
    response_derives = "Debug"
)]
pub struct NumQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.graphql",
    query_path = "src/gql/name_query.graphql",
    response_derives = "Debug"
)]
pub struct NameQuery;

#[tokio::main]
pub async fn fetch_dex_num(num: num_query::Variables) -> Result<graphql_client::Response<num_query::ResponseData>, Box<dyn Error>> {
    let request_body = NumQuery::build_query(num);

    let client = reqwest::Client::new();
    let res = client.post("https://graphqlpokemon.favware.tech/").json(&request_body).send().await?;
    let response_body: Response<num_query::ResponseData> = res.json().await?;
    Ok(response_body)
}

#[tokio::main]
async fn fetch_dex_name(name: name_query::Variables) -> Result<graphql_client::Response<name_query::ResponseData>, Box<dyn Error>> {
    let request_body = NameQuery::build_query(name);

    let client = reqwest::Client::new();
    let res = client.post("https://graphqlpokemon.favware.tech/").json(&request_body).send().await?;
    let response_body: Response<name_query::ResponseData> = res.json().await?;
    Ok(response_body)
}

fn show_numresult(response: graphql_client::Response<num_query::ResponseData>) {
	let mon = response.data.unwrap().get_pokemon_by_dex_number;

	let mut table = Table::new();
    table
		.set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
		.set_table_width(80)
		.add_row(vec![
            "Species:",
            &mon.species,
        ])
		.add_row(vec![
			"Num:",
			&format!("#{}", mon.num),
		])
		.add_row(vec![
			"Types:",
			&mon.types.join(",").replace(",", " / "),
		])
		.add_row(vec![
			"Abilities:",
			&mon.abilities.first,
		])
		.add_row(vec![
			"Height:",
			&format!("{} M", mon.height),
		])
		.add_row(vec![
			"Weight:",
			&format!("{} KG", mon.weight),
		])
		.add_row(vec![
			"Description:",
			&mon.flavor_texts.get(0).unwrap().flavor.clone(),
		]);

    println!("{}", table);
}
fn show_nameresult(response: graphql_client::Response<name_query::ResponseData>) {
	let res = response.data.unwrap();
	let mon = res.get_fuzzy_pokemon.get(0).unwrap();

	let mut table = Table::new();
    table
		.set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
		.set_table_width(80)
		.add_row(vec![
            "Species:",
            &mon.species,
        ])
		.add_row(vec![
			"Num:",
			&format!("#{}", mon.num),
		])
		.add_row(vec![
			"Types:",
			&mon.types.join(",").replace(",", " / "),
		])
		.add_row(vec![
			"Abilities:",
			&mon.abilities.first,
		])
		.add_row(vec![
			"Height:",
			&format!("{} M", mon.height),
		])
		.add_row(vec![
			"Weight:",
			&format!("{} KG", mon.weight),
		])
		.add_row(vec![
			"Description:",
			&mon.flavor_texts.get(0).unwrap().flavor.clone(),
		]);

    println!("{}", table);
}

pub fn run() {
    let yaml = load_yaml!("config/cli.yaml");
    let matches = App::from(yaml).get_matches();

    if matches.is_present("num") {
        let dexnum = num_query::Variables{
            num: matches.value_of("num").unwrap().parse::<i64>().unwrap()
        };
        let response = fetch_dex_num(dexnum).expect("Query unsuccessful!");
		show_numresult(response);
    } else if matches.is_present("name"){
        let dexname = name_query::Variables{
            pokemon: String::from(matches.value_of("name").unwrap())
        };
        let response = fetch_dex_name(dexname).expect("Query unsuccessful!");
		show_nameresult(response);
    } else {
        println!("Please provide an argument!");
    }
}
