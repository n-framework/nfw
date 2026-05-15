use crate::features::generator_management::models::listed_generator::ListedGenerator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListGeneratorsQueryResult {
    pub generators: Vec<ListedGenerator>,
    pub warnings: Vec<String>,
}
