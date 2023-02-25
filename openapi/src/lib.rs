
pub struct OpenAPI {
    openapi: String,
    info: Info,
    servers: Option<Vec<Server>>,
    paths: Paths,
    components: Option<Components>,
    security: Option<SecurityRequirement>,
    tags: Option<Vec<Tag>>,
    external_docs: Option<ExternalDocumentation>
}
