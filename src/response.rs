use comfy_table::Table;
use crate::fetch;

pub fn show_numresult(response: graphql_client::Response<fetch::num_query::ResponseData>) {
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

pub fn show_nameresult(response: graphql_client::Response<fetch::name_query::ResponseData>) {
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