namespace NFramework.NFW.Application.Features.Templates;

public sealed class TemplateCatalogException : Exception
{
    public TemplateCatalogException(string message)
        : base(message) { }

    public TemplateCatalogException(string message, Exception innerException)
        : base(message, innerException) { }
}
