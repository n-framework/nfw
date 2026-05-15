use crate::features::generator_management::models::errors::generators_service_error::GeneratorsServiceError;
use crate::features::generator_management::queries::list_generators::list_generators_query::ListGeneratorsQuery;
use crate::features::generator_management::queries::list_generators::list_generators_query_result::ListGeneratorsQueryResult;
use crate::features::generator_management::services::abstractions::generator_listing_service::GeneratorListingService;

#[derive(Debug, Clone)]
pub struct ListGeneratorsQueryHandler<S>
where
    S: GeneratorListingService,
{
    generators_service: S,
}

impl<S> ListGeneratorsQueryHandler<S>
where
    S: GeneratorListingService,
{
    pub fn new(generators_service: S) -> Self {
        Self { generators_service }
    }

    pub fn handle(
        &self,
        _query: ListGeneratorsQuery,
    ) -> Result<ListGeneratorsQueryResult, GeneratorsServiceError> {
        let (generators, warnings) = self.generators_service.list_generators()?;
        Ok(ListGeneratorsQueryResult {
            generators,
            warnings,
        })
    }
}
