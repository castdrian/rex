use crate::fetch;
use crate::constants;
use crate::images;
use voca_rs::*;
use colored::Colorize;
use term_table::table_cell::TableCell;
use term_table::row::Row;

pub fn show_numresult(response: graphql_client::Response<fetch::num_query::ResponseData>) {
	let mon = response.data.unwrap().get_pokemon_by_dex_number;
	
	let mut table = term_table::Table::new();
	table.max_column_width = 35;

	table.style = term_table::TableStyle::extended();
	table.add_row(Row::new(vec![
		TableCell::new("Species:"),
		TableCell::new(format!("#{} {}", mon.num, case::capitalize(&mon.species, true))),
	]));
	table.add_row(Row::new(vec![
		TableCell::new("Types:"),
		TableCell::new(format!("{}{}", format!("{:\u{2009}^10}", constants::assign_typecolor(mon.types.get(0).unwrap())), if mon.types.len() > 1 { format!("{:\u{2009}^10}", constants::assign_typecolor(mon.types.get(1).unwrap())) } else { format!("") })),
	]));
	table.add_row(Row::new(vec![
		TableCell::new("Abilities:"),
		TableCell::new(format!("{}{}{}", mon.abilities.first, if mon.abilities.second == None { format!("") } else { format!(" / {}", mon.abilities.second.as_ref().unwrap()) }, if mon.abilities.hidden == None { format!("") } else { format!(" | HA: {}", mon.abilities.hidden.as_ref().unwrap()) })),
	]));
	table.add_row(Row::new(vec![
		TableCell::new("Dimensions:"),
		TableCell::new(format!("Height: {} M | Weight: {} KG", mon.height, mon.weight)),
	]));
	table.add_row(Row::new(vec![
		TableCell::new("Description:"),
		TableCell::new_with_col_span(mon.flavor_texts.get(0).unwrap().flavor.clone(), 1),
	]));
	println!("{}", table.render());
}

pub fn show_nameresult(response: graphql_client::Response<fetch::name_query::ResponseData>) {
	let res = response.data.unwrap();
	let mon = res.get_fuzzy_pokemon.get(0).unwrap();

	let mut table = term_table::Table::new();
	table.max_column_width = 35;

	table.style = term_table::TableStyle::extended();
	table.add_row(Row::new(vec![
		TableCell::new("Species:"),
		TableCell::new(format!("#{} {} {}: {} {}: {}", mon.num, case::capitalize(&mon.species, true), "♂".blue().bold(), mon.gender.male, "♀".red().bold(), mon.gender.female)),
	]));
	table.add_row(Row::new(vec![
		TableCell::new("Types:"),
		TableCell::new(format!("{}{}", format!("{:\u{2009}^10}", constants::assign_typecolor(mon.types.get(0).unwrap())), if mon.types.len() > 1 { format!("{:\u{2009}^10}", constants::assign_typecolor(mon.types.get(1).unwrap())) } else { format!("") })),
	]));
	table.add_row(Row::new(vec![
		TableCell::new("Abilities:"),
		TableCell::new(format!("{}{}{}", mon.abilities.first, if mon.abilities.second == None { format!("") } else { format!(" / {}", mon.abilities.second.as_ref().unwrap()) }, if mon.abilities.hidden == None { format!("") } else { format!(" | HA: {}", mon.abilities.hidden.as_ref().unwrap()) })),
	]));
	table.add_row(Row::new(vec![
		TableCell::new("Dimensions:"),
		TableCell::new(format!("Height: {} M | Weight: {} KG", mon.height, mon.weight)),
	]));
	table.add_row(Row::new(vec![
		TableCell::new("Description:"),
		TableCell::new_with_col_span(mon.flavor_texts.get(0).unwrap().flavor.clone(), 1),
	]));
	println!("{}", table.render());
	images::show_sprite(&mon.sprite);
}