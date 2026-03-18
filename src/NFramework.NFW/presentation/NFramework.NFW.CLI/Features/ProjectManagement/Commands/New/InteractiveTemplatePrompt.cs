using NFramework.NFW.Application.Features.TemplateManagement.Queries.ListTemplates;
using NFramework.NFW.CLI.Features.ProjectManagement.Commands.New.Abstractions;
using Spectre.Console;

namespace NFramework.NFW.CLI.Features.ProjectManagement.Commands.New;

public sealed class InteractiveTemplatePrompt(IAnsiConsole console)
{
    private readonly IAnsiConsole _console = console;

    public TerminalTemplateSelectionResult PromptForTemplateSelection(IReadOnlyList<ListedTemplate> templates)
    {
        SelectionPrompt<ListedTemplate> prompt = new SelectionPrompt<ListedTemplate>()
            .Title("Select a template")
            .UseConverter(template => $"{template.Identifier} | {template.DisplayName} | {template.Description}")
            .AddChoices(templates);

        try
        {
            ListedTemplate selectedTemplate = _console.Prompt(prompt);
            return TerminalTemplateSelectionResult.Selected(selectedTemplate.Identifier);
        }
        catch (OperationCanceledException)
        {
            return TerminalTemplateSelectionResult.Cancelled();
        }
        catch (InvalidOperationException)
        {
            return TerminalTemplateSelectionResult.Cancelled();
        }
    }

    public TerminalTextInputResult PromptForWorkspaceName()
    {
        TextPrompt<string> prompt = new TextPrompt<string>("Workspace name:")
            .PromptStyle("green")
            .Validate(
                (string workspaceName) =>
                    string.IsNullOrWhiteSpace(workspaceName)
                        ? ValidationResult.Error("[red]Workspace name is required.[/]")
                        : ValidationResult.Success()
            );

        try
        {
            string workspaceName = _console.Prompt(prompt);
            return TerminalTextInputResult.Submitted(workspaceName);
        }
        catch (OperationCanceledException)
        {
            return TerminalTextInputResult.Cancelled();
        }
        catch (InvalidOperationException)
        {
            return TerminalTextInputResult.Cancelled();
        }
    }
}
