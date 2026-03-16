using NFramework.NFW.Domain.Features.Templates;
using YamlDotNet.Serialization;

namespace NFramework.NFW.Application.Features.Templates;

public sealed class TemplateCatalogParser
{
    private readonly IDeserializer _deserializer;

    public TemplateCatalogParser()
    {
        _deserializer = new DeserializerBuilder().IgnoreUnmatchedProperties().Build();
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
                .Select(template => new TemplateDescriptor(
                    template.Name!.Trim(),
                    string.IsNullOrWhiteSpace(template.Description)
                        ? "No description provided."
                        : template.Description.Trim()
                ))
                .ToArray();
        }
        catch (Exception exception)
        {
            throw new TemplateCatalogException("Template catalog content is invalid.", exception);
        }
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
