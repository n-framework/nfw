use crate::features::check::services::abstractions::{
    ExternalToolRunner, NamespaceUsageValidator, PackageUsageValidator, ProjectManifestCollector,
    ProjectReferenceValidator, SourceFileCollector, WorkspaceMetadataReader,
};
use crate::features::check::services::finding_aggregation_service::FindingAggregationService;
use crate::features::check::services::remediation_hint_service::RemediationHintService;

/// Group of validation services for dependency injection.
/// This reduces the number of parameters in CheckCommandHandler::new from 6 to 3.
pub struct ValidationServices {
    // Validator services (as trait objects)
    pub project_reference_validator: Box<dyn ProjectReferenceValidator>,
    pub namespace_usage_validator: Box<dyn NamespaceUsageValidator>,
    pub package_usage_validator: Box<dyn PackageUsageValidator>,
    // Infrastructure services (as trait objects)
    pub manifest_collector: Box<dyn ProjectManifestCollector>,
    pub source_file_collector: Box<dyn SourceFileCollector>,
    pub workspace_metadata_reader: Box<dyn WorkspaceMetadataReader>,
    pub external_tool_runner: Box<dyn ExternalToolRunner>,
    // Support services (concrete types)
    pub finding_aggregation_service: FindingAggregationService,
    pub remediation_hint_service: RemediationHintService,
}

impl ValidationServices {
    pub fn builder() -> ValidationServicesBuilder {
        ValidationServicesBuilder::default()
    }
}

#[derive(Default)]
pub struct ValidationServicesBuilder {
    project_reference_validator: Option<Box<dyn ProjectReferenceValidator>>,
    namespace_usage_validator: Option<Box<dyn NamespaceUsageValidator>>,
    package_usage_validator: Option<Box<dyn PackageUsageValidator>>,
    manifest_collector: Option<Box<dyn ProjectManifestCollector>>,
    source_file_collector: Option<Box<dyn SourceFileCollector>>,
    workspace_metadata_reader: Option<Box<dyn WorkspaceMetadataReader>>,
    external_tool_runner: Option<Box<dyn ExternalToolRunner>>,
    finding_aggregation_service: Option<FindingAggregationService>,
    remediation_hint_service: Option<RemediationHintService>,
}

impl ValidationServicesBuilder {
    pub fn project_reference_validator(
        mut self,
        value: Box<dyn ProjectReferenceValidator>,
    ) -> Self {
        self.project_reference_validator = Some(value);
        self
    }

    pub fn namespace_usage_validator(mut self, value: Box<dyn NamespaceUsageValidator>) -> Self {
        self.namespace_usage_validator = Some(value);
        self
    }

    pub fn package_usage_validator(mut self, value: Box<dyn PackageUsageValidator>) -> Self {
        self.package_usage_validator = Some(value);
        self
    }

    pub fn manifest_collector(mut self, value: Box<dyn ProjectManifestCollector>) -> Self {
        self.manifest_collector = Some(value);
        self
    }

    pub fn source_file_collector(mut self, value: Box<dyn SourceFileCollector>) -> Self {
        self.source_file_collector = Some(value);
        self
    }

    pub fn workspace_metadata_reader(mut self, value: Box<dyn WorkspaceMetadataReader>) -> Self {
        self.workspace_metadata_reader = Some(value);
        self
    }

    pub fn external_tool_runner(mut self, value: Box<dyn ExternalToolRunner>) -> Self {
        self.external_tool_runner = Some(value);
        self
    }

    pub fn finding_aggregation_service(mut self, value: FindingAggregationService) -> Self {
        self.finding_aggregation_service = Some(value);
        self
    }

    pub fn remediation_hint_service(mut self, value: RemediationHintService) -> Self {
        self.remediation_hint_service = Some(value);
        self
    }

    pub fn build(self) -> ValidationServices {
        ValidationServices {
            project_reference_validator: self
                .project_reference_validator
                .expect("project_reference_validator is required"),
            namespace_usage_validator: self
                .namespace_usage_validator
                .expect("namespace_usage_validator is required"),
            package_usage_validator: self
                .package_usage_validator
                .expect("package_usage_validator is required"),
            manifest_collector: self
                .manifest_collector
                .expect("manifest_collector is required"),
            source_file_collector: self
                .source_file_collector
                .expect("source_file_collector is required"),
            workspace_metadata_reader: self
                .workspace_metadata_reader
                .expect("workspace_metadata_reader is required"),
            external_tool_runner: self
                .external_tool_runner
                .expect("external_tool_runner is required"),
            finding_aggregation_service: self
                .finding_aggregation_service
                .expect("finding_aggregation_service is required"),
            remediation_hint_service: self
                .remediation_hint_service
                .expect("remediation_hint_service is required"),
        }
    }
}
