use crate::fetch;
use crate::fetch::name_query::NameQueryGetFuzzyPokemon;
use crate::fetch::num_query::NumQueryGetPokemonByDexNumber;

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
