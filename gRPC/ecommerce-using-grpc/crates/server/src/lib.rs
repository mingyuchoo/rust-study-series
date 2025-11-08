use proto::product_info_server::ProductInfo;
use proto::{Product, ProductId};
use thiserror::Error;
use tonic::{Request, Response, Status};

// Define custom error types for Railway Oriented Programming
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Product not found: {0}")]
    NotFound(i32),

    #[error("Invalid product data: {0}")]
    InvalidData(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

// Implement conversion from ServiceError to tonic::Status
impl From<ServiceError> for Status {
    fn from(error: ServiceError) -> Self {
        match error {
            | ServiceError::NotFound(id) => Status::not_found(format!("Product with id {} not found", id)),
            | ServiceError::InvalidData(msg) => Status::invalid_argument(msg),
            | ServiceError::Internal(msg) => Status::internal(msg),
        }
    }
}

// Type alias for service results to simplify Railway Oriented Programming
pub type ServiceResult<T> = Result<T, ServiceError>;

// Product service implementation
#[derive(Debug, Default)]
pub struct MyProductInfo {}

impl MyProductInfo {
    // Internal method to add a product with Railway Oriented Programming
    async fn add_product_internal(&self, product: Product) -> ServiceResult<ProductId> {
        // Validate product data
        if product.name.is_empty() {
            return Err(ServiceError::InvalidData("Product name cannot be empty".to_string()));
        }

        if product.price <= 0.0 {
            return Err(ServiceError::InvalidData("Product price must be positive".to_string()));
        }

        // In a real application, this would interact with a database
        // For now, we just echo back the ID
        Ok(ProductId {
            id: product.id,
        })
    }

    // Internal method to get a product with Railway Oriented Programming
    async fn get_product_internal(&self, product_id: ProductId) -> ServiceResult<Product> {
        // In a real application, this would query a database
        // For demonstration, we return a hardcoded product if ID > 0
        if product_id.id <= 0 {
            return Err(ServiceError::NotFound(product_id.id));
        }

        Ok(Product {
            id: product_id.id,
            name: String::from("MacBook Air 15"),
            description: String::from("Impressively big. Impossibly thin."),
            price: 1299.9,
        })
    }
}

#[tonic::async_trait]
impl ProductInfo for MyProductInfo {
    async fn add_product(&self, request: Request<Product>) -> Result<Response<ProductId>, Status> {
        // Extract the product from the request
        let product = request.into_inner();

        // Use Railway Oriented Programming pattern
        self.add_product_internal(product).await.map(Response::new).map_err(Into::into) // Convert ServiceError to Status
    }

    async fn get_product(&self, request: Request<ProductId>) -> Result<Response<Product>, Status> {
        // Extract the product ID from the request
        let product_id = request.into_inner();

        // Use Railway Oriented Programming pattern
        self.get_product_internal(product_id).await.map(Response::new).map_err(Into::into) // Convert ServiceError to Status
    }
}
