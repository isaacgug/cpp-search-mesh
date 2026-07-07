use thiserror::Error;

#[derive(Debug, Error)]
enum ServerError {
    #[error("stdio MCP server is not implemented yet")]
    NotImplemented,
}

fn main() -> Result<(), ServerError> {
    Err(ServerError::NotImplemented)
}
