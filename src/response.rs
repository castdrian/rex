use crate::constants;
use crate::fetch;
use crate::fetch::name_query::NameQueryGetFuzzyPokemon;
use crate::fetch::num_query::NumQueryGetPokemonByDexNumber;
use crate::images;
use colored::Colorize;
use term_table::row::Row;
use term_table::table_cell::TableCell;
use voca_rs::*;

pub fn cli_show_numresult(response: graphql_client::Response<fetch::num_query::ResponseData>) {
    let mon = response.data.unwrap().get_pokemon_by_dex_number;

    let mut table = term_table::Table::new();
    table.max_column_width = 50;

    table.style = term_table::TableStyle::extended();
    table.add_row(Row::new(vec![
        TableCell::new(format!("{:15}", "")),
        TableCell::new_with_col_span(
            format!("{:300}", mon.flavor_texts.get(0).unwrap().flavor.clone()),
            1,
        ),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("Species:"),
        TableCell::new(format!(
            "#{} {} {}: {} {}: {}",
            mon.num,
            case::capitalize(&mon.species, true),
            "♂".blue().bold(),
            mon.gender.male,
            "♀".red().bold(),
            mon.gender.female
        )),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("Types:"),
        TableCell::new(format!(
            "{}{}",
            format!(
                "{:\u{2009}^10}",
                constants::assign_typecolor(mon.types.get(0).unwrap().primary.as_str())
            ),
            if mon.types.len() > 1 {
                format!(
                    "{:\u{2009}^10}",
                    constants::assign_typecolor(mon.types.get(1).unwrap().secondary.as_str())
                )
            } else {
                format!("")
            }
        )),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("Abilities:"),
        TableCell::new(format!(
            "{}{}{}",
            mon.abilities.first.name,
            if mon.abilities.second.is_none() {
                format!("")
            } else {
                format!(" / {}", mon.abilities.second.as_ref().unwrap().name)
            },
            if mon.abilities.hidden.is_none() {
                format!("")
            } else {
                format!(" | HA: {}", mon.abilities.hidden.as_ref().unwrap().name)
            }
        )),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("Dimensions:"),
        TableCell::new(format!(
            "Height: {} M | Weight: {} KG",
            mon.height, mon.weight
        )),
    ]));
    println!("{}", table.render());

    images::show_sprite(&mon.sprite, Some(14), Some(7), 3, -17);
}

pub fn cli_show_nameresult(response: graphql_client::Response<fetch::name_query::ResponseData>) {
    let res = response.data.unwrap();
    let mon = res.get_fuzzy_pokemon.get(0).unwrap();

    let mut table = term_table::Table::new();
    table.max_column_width = 50;

    table.style = term_table::TableStyle::extended();
    table.add_row(Row::new(vec![
        TableCell::new(format!("{:15}", "")),
        TableCell::new_with_col_span(
            format!("{:300}", mon.flavor_texts.get(0).unwrap().flavor.clone()),
            1,
        ),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("Species:"),
        TableCell::new(format!(
            "#{} {} {}: {} {}: {}",
            mon.num,
            case::capitalize(&mon.species, true),
            "♂".blue().bold(),
            mon.gender.male,
            "♀".red().bold(),
            mon.gender.female
        )),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("Types:"),
        TableCell::new(format!(
            "{}{}",
            format!(
                "{:\u{2009}^10}",
                constants::assign_typecolor(mon.types.get(0).unwrap().primary.as_str())
            ),
            if mon.types.len() > 1 {
                format!(
                    "{:\u{2009}^10}",
                    constants::assign_typecolor(mon.types.get(1).unwrap().secondary.as_str())
                )
            } else {
                format!("")
            }
        )),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("Abilities:"),
        TableCell::new(format!(
            "{}{}{}",
            mon.abilities.first.name,
            if mon.abilities.second.is_none() {
                format!("")
            } else {
                format!(" / {}", mon.abilities.second.as_ref().unwrap().name)
            },
            if mon.abilities.hidden.is_none() {
                format!("")
            } else {
                format!(" | HA: {}", mon.abilities.hidden.as_ref().unwrap().name)
            }
        )),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("Dimensions:"),
        TableCell::new(format!(
            "Height: {} M | Weight: {} KG",
            mon.height, mon.weight
        )),
    ]));
    println!("{}", table.render());

    images::show_sprite(&mon.sprite, Some(14), Some(7), 3, -17);
}

pub fn gui_get_numresult(
    response: graphql_client::Response<fetch::num_query::ResponseData>,
) -> NumQueryGetPokemonByDexNumber {
    return response.data.unwrap().get_pokemon_by_dex_number;
}
pub fn gui_get_nameresult(
    response: graphql_client::Response<fetch::name_query::ResponseData>,
) -> NameQueryGetFuzzyPokemon {
    return response
        .data
        .unwrap()
        .get_fuzzy_pokemon
        .get(0)
        .unwrap()
        .clone();
}
