using NFramework.NFW.Domain.Features.TemplateManagement.ValueObjects;
using YamlDotNet.Core;
using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;

namespace NFramework.NFW.Application.Features.TemplateManagement.Services;

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

    public TemplateCatalog Parse(string catalogContent)
    {
        if (string.IsNullOrWhiteSpace(catalogContent))
            return TemplateCatalog.Empty;

        try
        {
            TemplateCatalogDocument document = _deserializer.Deserialize<TemplateCatalogDocument>(catalogContent);
            if (document.Templates is null || document.Templates.Count == 0)
                return TemplateCatalog.Empty;

            TemplateDescriptor[] entries = document
                .Templates.Select(CreateTemplateDescriptor)
                .Where(template => template is not null)
                .Cast<TemplateDescriptor>()
                .ToArray();

            return TemplateCatalog.Create(entries);
        }
        catch (YamlException exception)
        {
            string message = BuildYamlErrorMessage(exception);
            throw new TemplateCatalogException($"Template catalog has invalid YAML syntax. {message}", exception);
        }
        catch (TemplateCatalogException)
        {
            throw;
        }
        catch (ArgumentException exception)
        {
            throw new TemplateCatalogException(exception.Message, exception);
        }
        catch (Exception exception)
        {
            throw new TemplateCatalogException("Failed to parse template catalog.", exception);
        }
    }

    private static TemplateDescriptor? CreateTemplateDescriptor(TemplateCatalogItem template)
    {
        string? identifier = template.Identifier ?? template.Name;
        if (string.IsNullOrWhiteSpace(identifier))
            return null;

        string? displayName = template.DisplayName ?? template.Name ?? template.Identifier;
        return TemplateDescriptor.Create(identifier, displayName, template.Description);
    }

    private static string BuildYamlErrorMessage(YamlException exception)
    {
        Mark start = exception.Start;
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
        public string? Identifier { get; init; }

        public string? Name { get; init; }

        public string? DisplayName { get; init; }

        public string? Description { get; init; }
    }
}
