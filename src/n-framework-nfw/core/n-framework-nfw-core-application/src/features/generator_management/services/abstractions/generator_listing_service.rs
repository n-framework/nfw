use crate::features::generator_management::models::errors::generators_service_error::GeneratorsServiceError;
use crate::features::generator_management::models::listed_generator::ListedGenerator;

pub trait GeneratorListingService {
    fn list_generators(
        &self,
    ) -> Result<(Vec<ListedGenerator>, Vec<String>), GeneratorsServiceError>;
}
