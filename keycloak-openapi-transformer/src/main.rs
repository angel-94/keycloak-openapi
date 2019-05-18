use openapiv3::OpenAPI;
use scraper::Html;
use serde_json::to_string_pretty;
use std::collections::BTreeMap;

const HTML: &str = include_str!("../../keycloak/6.0.html");

mod components;
mod info;

fn main() -> Result<(), Box<std::error::Error>> {
    let document = Html::parse_document(HTML);
    let spec = OpenAPI {
        openapi: "3.0.2".to_string(),
        info: info::parse(&document)?,
        servers: vec![],
        paths: BTreeMap::new(),
        components: Some(openapiv3::Components {
            security_schemes: BTreeMap::new(),
            responses: BTreeMap::new(),
            parameters: BTreeMap::new(),
            examples: BTreeMap::new(),
            request_bodies: BTreeMap::new(),
            headers: BTreeMap::new(),
            schemas: components::schemas::parse_schemas(&document),
            links: BTreeMap::new(),
            callbacks: BTreeMap::new(),
        }),
        security: vec![],
        tags: vec![],
        external_docs: Some(openapiv3::ExternalDocumentation {
            description: Some("Schema source code".to_string()),
            url: "https://github.com/keycloak/keycloak/tree/6.0.1/core/src/main/java/org/keycloak/representations".to_string()
        }),
    };

    println!("{}", to_string_pretty(&spec)?);
    Ok(())
}