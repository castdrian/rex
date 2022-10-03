use crate::fetch;
use crate::fetch::name_query::NameQueryGetFuzzyPokemon;

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
