namespace NFramework.NFW.Domain.Features.Templates;

public sealed record TemplateDescriptor(string Name, string Description)
{
    public static TemplateDescriptor Create(string name, string? description = null)
    {
        if (string.IsNullOrWhiteSpace(name))
        {
            throw new ArgumentException("Name cannot be empty or whitespace.", nameof(name));
        }

        var trimmedName = name.Trim();
        var trimmedDescription = string.IsNullOrWhiteSpace(description)
            ? "No description provided."
            : description.Trim();

        return new TemplateDescriptor(trimmedName, trimmedDescription);
    }
}
