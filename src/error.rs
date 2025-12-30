use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Unable to fetch data")]
    FetchError(#[from] rust_mc_status::McError),
    
}

#[derive(Error, Debug)]
pub enum AgregationError {

}
