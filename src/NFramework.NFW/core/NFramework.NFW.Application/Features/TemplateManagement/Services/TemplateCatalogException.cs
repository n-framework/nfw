namespace NFramework.NFW.Application.Features.TemplateManagement.Services;

public sealed class TemplateCatalogException : Exception
{
    public TemplateCatalogException(string message)
        : base(message) { }

    public TemplateCatalogException(string message, Exception innerException)
        : base(message, innerException) { }
}
