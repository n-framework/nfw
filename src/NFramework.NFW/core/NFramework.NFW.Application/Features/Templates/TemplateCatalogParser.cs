using NFramework.NFW.Domain.Features.Templates;
using YamlDotNet.Core;
using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;

namespace NFramework.NFW.Application.Features.Templates;

public sealed class TemplateCatalogParser
{
    private readonly IDeserializer _deserializer;

    public TemplateCatalogParser()
    {
        _deserializer = new DeserializerBuilder()
            .IgnoreUnmatchedProperties()
            .WithNamingConvention(CamelCaseNamingConvention.Instance)
            .Build();
    }

    public IReadOnlyList<TemplateDescriptor> Parse(string catalogContent)
    {
        if (string.IsNullOrWhiteSpace(catalogContent))
        {
            return [];
        }

        try
        {
            var document = _deserializer.Deserialize<TemplateCatalogDocument>(catalogContent);
            if (document.Templates is null || document.Templates.Count == 0)
            {
                return [];
            }

            return document
                .Templates.Where(template => !string.IsNullOrWhiteSpace(template.Name))
                .Select(template => TemplateDescriptor.Create(template.Name!, template.Description))
                .ToArray();
        }
        catch (YamlException exception)
        {
            var message = BuildYamlErrorMessage(exception);
            throw new TemplateCatalogException($"Template catalog has invalid YAML syntax. {message}", exception);
        }
        catch (TemplateCatalogException)
        {
            throw;
        }
        catch (Exception exception)
        {
            throw new TemplateCatalogException("Failed to parse template catalog.", exception);
        }
    }

    private static string BuildYamlErrorMessage(YamlException exception)
    {
        var start = exception.Start;
        if (start.Line == 0 && start.Column == 0)
        {
            return exception.Message;
        }

        return $"Line {start.Line}, Column {start.Column}: {exception.Message}";
    }

    private sealed class TemplateCatalogDocument
    {
        public List<TemplateCatalogItem>? Templates { get; init; }
    }

    private sealed class TemplateCatalogItem
    {
        public string? Name { get; init; }

        public string? Description { get; init; }
    }
}
