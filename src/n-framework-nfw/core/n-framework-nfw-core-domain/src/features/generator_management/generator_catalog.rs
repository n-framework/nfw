use crate::features::generator_management::generator_descriptor::GeneratorDescriptor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratorCatalog {
    pub source_name: String,
    pub generators: Vec<GeneratorDescriptor>,
}

impl GeneratorCatalog {
    pub fn new(source_name: String, generators: Vec<GeneratorDescriptor>) -> Self {
        Self {
            source_name,
            generators,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.generators.is_empty()
    }

    pub fn len(&self) -> usize {
        self.generators.len()
    }
}
