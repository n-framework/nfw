using NFramework.NFW.Application.Features.Cli.Logging;
using NFramework.NFW.Application.Features.TemplateManagement.Queries.ListTemplates;
using NFramework.NFW.Application.Features.TemplateManagement.Services;
using NFramework.NFW.Application.Features.TemplateManagement.Services.Abstractions;
using NFramework.NFW.Application.Features.Versioning.Abstractions;
using NFramework.NFW.Domain.Features.Version;
using Xunit;

namespace NFramework.NFW.CLI.Tests.core.NFramework.NFW.Application.Features.TemplateManagement.Queries.ListTemplates;

public class ListTemplatesQueryHandlerTests
{
    [Fact]
    public async Task HandleAsync_MapsCatalogEntriesInCatalogOrder()
    {
        ListTemplatesQueryHandler handler = CreateHandler(
            """
            templates:
              - identifier: blank
                displayName: Blank Workspace
                description: Minimal starter
              - identifier: minimal
                displayName: Minimal API
                description: API starter
            """
        );

        ListTemplatesQueryResult result = await handler.HandleAsync(new ListTemplatesQuery(), CancellationToken.None);

        result.IsSuccess.ShouldBeTrue();
        IReadOnlyList<ListedTemplate> templates = result.Templates.ShouldNotBeNull();
        templates.Count.ShouldBe(2);
        templates[0].Identifier.ShouldBe("blank");
        templates[1].Identifier.ShouldBe("minimal");
    }

    [Fact]
    public async Task HandleAsync_WithEmptyCatalog_ReturnsSuccessfulEmptyList()
    {
        ListTemplatesQueryResult result = await CreateHandler("templates: []")
            .HandleAsync(new ListTemplatesQuery(), CancellationToken.None);

        result.IsSuccess.ShouldBeTrue();
        result.Templates.ShouldNotBeNull().ShouldBeEmpty();
    }

    private static ListTemplatesQueryHandler CreateHandler(string catalogContent)
    {
        ILocalTemplateCatalogSource localSource = new UnavailableLocalTemplateCatalogSource();
        return new ListTemplatesQueryHandler(
            new TemplatesService(
                new TemplateCatalogParser(),
                localSource,
                new FakeRemoteTemplateCatalogSource(catalogContent),
                new TemplateCatalogSourceResolver(localSource)
            ),
            new FakeVersionProvider(),
            new DiagnosticLogger()
        );
    }

    private sealed class UnavailableLocalTemplateCatalogSource : ILocalTemplateCatalogSource
    {
        public string? ReadCatalog() => null;

        public string? TryGetCatalogPath() => null;
    }

    private sealed class FakeRemoteTemplateCatalogSource(string catalogContent) : IRemoteTemplateCatalogSource
    {
        public Task<string> FetchCatalogAsync(string cliVersion, CancellationToken cancellationToken)
        {
            return Task.FromResult(catalogContent);
        }
    }

    private sealed class FakeVersionProvider : IVersionProvider
    {
        public VersionInfo GetVersionInfo()
        {
            return VersionInfo.Create("1.2.3");
        }
    }
}
