use graphql_client::GraphQLQuery;
use poll_promise::Promise;
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.graphql",
    query_path = "src/gql/num_query.graphql",
    response_derives = "Debug, Clone"
)]
pub struct NumQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.graphql",
    query_path = "src/gql/name_query.graphql",
    response_derives = "Debug, Clone"
)]
pub struct NameQuery;

pub fn fetch_dex_num(
    num: num_query::Variables,
) -> Result<graphql_client::Response<num_query::ResponseData>, Box<dyn Error>> {
    let mut promise: Option<Promise<Vec<u8>>> = None;

    let result = promise.get_or_insert_with(|| {
        let (sender, promise) = Promise::new();
        let request_body = NumQuery::build_query(num);
        let request_body = serde_json::to_vec(&request_body);

        let request = ehttp::Request {
            headers: ehttp::headers(&[("Accept", "*/*"), ("Content-Type", "application/json")]),
            ..ehttp::Request::post(
                "https://graphqlpokemon.favware.tech/v7",
                request_body.unwrap(),
            )
        };

        ehttp::fetch(request, move |response| {
            let bytes = response.unwrap().bytes.to_vec();
            sender.send(bytes);
        });
        promise
    });

    let response_bytes = result.block_until_ready().clone();
    let body = serde_json::from_slice::<graphql_client::Response<num_query::ResponseData>>(
        &response_bytes,
    )
    .unwrap();
    Ok(body)
}

pub fn fetch_dex_name(
    name: name_query::Variables,
) -> Result<graphql_client::Response<name_query::ResponseData>, Box<dyn Error>> {
    let mut promise: Option<Promise<Vec<u8>>> = None;

    let result = promise.get_or_insert_with(|| {
        let (sender, promise) = Promise::new();
        let request_body = NameQuery::build_query(name);
        let request_body = serde_json::to_vec(&request_body);

        let request = ehttp::Request {
            headers: ehttp::headers(&[("Accept", "*/*"), ("Content-Type", "application/json")]),
            ..ehttp::Request::post(
                "https://graphqlpokemon.favware.tech/v7",
                request_body.unwrap(),
            )
        };

        ehttp::fetch(request, move |response| {
            let bytes = response.unwrap().bytes.to_vec();
            sender.send(bytes);
        });
        promise
    });

    let response_bytes = result.block_until_ready().clone();
    let body = serde_json::from_slice::<graphql_client::Response<name_query::ResponseData>>(
        &response_bytes,
    )
    .unwrap();
    Ok(body)
}
