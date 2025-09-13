pub mod azure_openai;
pub mod connection_pool;

#[cfg(test)]
mod tests;

pub use azure_openai::AzureOpenAIClient;
#[allow(unused_imports)]
pub use connection_pool::*;
