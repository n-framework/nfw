use n_framework_nfw_core_domain::features::generator_management::generator_catalog::GeneratorCatalog;

use crate::features::generator_management::models::errors::generators_service_error::GeneratorsServiceError;

pub trait GeneratorCatalogDiscoveryService {
    fn discover_catalogs(
        &self,
    ) -> Result<(Vec<GeneratorCatalog>, Vec<String>), GeneratorsServiceError>;
}
