namespace NFramework.NFW.Domain.Features.TemplateManagement.ValueObjects;

public sealed class TemplateCatalog
{
    private TemplateCatalog(IReadOnlyList<TemplateDescriptor> entries)
    {
        Entries = entries;
    }

    public static TemplateCatalog Empty { get; } = new([]);

    public IReadOnlyList<TemplateDescriptor> Entries { get; }

    public static TemplateCatalog Create(IEnumerable<TemplateDescriptor> entries)
    {
        ArgumentNullException.ThrowIfNull(entries);

        TemplateDescriptor[] orderedEntries = entries.ToArray();
        IGrouping<string, TemplateDescriptor>? duplicateIdentifier = orderedEntries
            .GroupBy(entry => entry.Identifier, StringComparer.OrdinalIgnoreCase)
            .FirstOrDefault(group => group.Count() > 1);

        if (duplicateIdentifier is not null)
        {
            throw new ArgumentException(
                $"Duplicate template identifier '{duplicateIdentifier.First().Identifier}' is not allowed.",
                nameof(entries)
            );
        }

        return orderedEntries.Length == 0 ? Empty : new TemplateCatalog(orderedEntries);
    }
}
