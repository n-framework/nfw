namespace NFramework.NFW.Domain.Features.TemplateManagement.ValueObjects;

public sealed record TemplateDescriptor(string Identifier, string DisplayName, string Description)
{
    public static TemplateDescriptor Create(string identifier, string? displayName = null, string? description = null)
    {
        if (string.IsNullOrWhiteSpace(identifier))
        {
            throw new ArgumentException("Identifier cannot be empty or whitespace.", nameof(identifier));
        }

        string trimmedIdentifier = identifier.Trim();
        string trimmedDisplayName = string.IsNullOrWhiteSpace(displayName) ? trimmedIdentifier : displayName.Trim();
        string trimmedDescription = string.IsNullOrWhiteSpace(description)
            ? "No description provided."
            : description.Trim();

        return new TemplateDescriptor(trimmedIdentifier, trimmedDisplayName, trimmedDescription);
    }
}
