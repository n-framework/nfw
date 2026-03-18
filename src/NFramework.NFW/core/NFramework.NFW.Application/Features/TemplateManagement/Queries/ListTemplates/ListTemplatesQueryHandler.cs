using NFramework.NFW.Application.Features.Cli.Logging;
using NFramework.NFW.Application.Features.TemplateManagement.Services;
using NFramework.NFW.Application.Features.Versioning.Abstractions;
using NFramework.NFW.Domain.Features.TemplateManagement.ValueObjects;
using NFramework.NFW.Domain.Features.Version;

namespace NFramework.NFW.Application.Features.TemplateManagement.Queries.ListTemplates;

public sealed class ListTemplatesQueryHandler(
    TemplatesService templatesService,
    IVersionProvider versionProvider,
    DiagnosticLogger diagnosticLogger
)
{
    private readonly TemplatesService _templatesService = templatesService;
    private readonly IVersionProvider _versionProvider = versionProvider;
    private readonly DiagnosticLogger _diagnosticLogger = diagnosticLogger;

    public async Task<ListTemplatesQueryResult> HandleAsync(
        ListTemplatesQuery query,
        CancellationToken cancellationToken
    )
    {
        ArgumentNullException.ThrowIfNull(query);

        try
        {
            _diagnosticLogger.Write("Resolving template catalog source.");
            VersionInfo versionInfo = _versionProvider.GetVersionInfo();
            TemplateCatalog catalog = await _templatesService.GetCatalogAsync(
                versionInfo.SemanticVersion,
                cancellationToken
            );
            ListedTemplate[] templates = catalog
                .Entries.Select(template => new ListedTemplate(
                    template.Identifier,
                    template.DisplayName,
                    template.Description
                ))
                .ToArray();

            return ListTemplatesQueryResult.Success(templates);
        }
        catch (TemplateCatalogException exception)
        {
            _diagnosticLogger.Write(exception.ToString());
            return ListTemplatesQueryResult.FailureResult(exception.Message);
        }
    }
}
