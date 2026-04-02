use nframework_nfw_application::features::cli::configuration::abstraction::config_store::ConfigStore;
use nframework_nfw_application::features::template_management::services::abstraction::git_repository::GitRepository;
use nframework_nfw_application::features::template_management::services::abstraction::template_catalog_source::TemplateCatalogSource;
use nframework_nfw_application::features::template_management::services::abstraction::template_source_synchronizer::TemplateSourceSynchronizer;
use nframework_nfw_application::features::template_management::services::abstraction::validator::Validator;
use nframework_nfw_application::features::template_management::services::abstraction::yaml_parser::YamlParser;
use nframework_nfw_application::features::template_management::services::templates_service::TemplatesService;
use nframework_nfw_application::features::versioning::abstraction::version_comparator::VersionComparator;

pub trait SourceManagementService {
    fn add_source(&self, name: &str, url: &str) -> Result<(), String>;
    fn remove_source(&self, name: &str) -> Result<(), String>;
}

impl<R, S, Y, V, C, CS, G> SourceManagementService for TemplatesService<R, S, Y, V, C, CS, G>
where
    R: TemplateSourceSynchronizer,
    S: TemplateCatalogSource,
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
    CS: ConfigStore,
    G: GitRepository,
{
    fn add_source(&self, name: &str, url: &str) -> Result<(), String> {
        TemplatesService::add_source(self, name, url).map_err(|error| error.to_string())
    }

    fn remove_source(&self, name: &str) -> Result<(), String> {
        TemplatesService::remove_source(self, name).map_err(|error| error.to_string())
    }
}
