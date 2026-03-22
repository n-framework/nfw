using NFramework.Core.CLI.Abstractions;
using NFramework.NFW.Application.Features.Cli;
using NFramework.NFW.Application.Features.ProjectManagement.Commands.New;
using NFramework.NFW.Application.Features.TemplateManagement.Queries.ListTemplates;

namespace NFramework.NFW.CLI.Features.ProjectManagement.Commands.New;

[CliCommand("new", "Create a new workspace from a template")]
public sealed class NewCliCommand(
    ListTemplatesQueryHandler listTemplatesQueryHandler,
    CreateWorkspaceCommandHandler createWorkspaceCommandHandler,
    ITerminalSession terminalSession
) : IAsyncCliCommand<NewCliCommandSettings>
{
    private readonly ListTemplatesQueryHandler _listTemplatesQueryHandler = listTemplatesQueryHandler;
    private readonly CreateWorkspaceCommandHandler _createWorkspaceCommandHandler = createWorkspaceCommandHandler;
    private readonly ITerminalSession _terminalSession = terminalSession;

    public async Task<int> ExecuteAsync(
        CliCommandContext context,
        NewCliCommandSettings settings,
        CancellationToken cancellationToken
    )
    {
        bool promptsAllowed = _terminalSession.IsInteractive && !settings.NoInput;

        WorkspaceNameResolutionResult workspaceNameResult = await ResolveWorkspaceNameAsync(
            settings.WorkspaceName,
            promptsAllowed,
            cancellationToken
        );
        if (!workspaceNameResult.IsSuccess)
        {
            _terminalSession.WriteErrorLine(workspaceNameResult.ErrorMessage!);
            return workspaceNameResult.ExitCode;
        }

        TemplateIdentifierResolutionResult templateIdentifierResult = await ResolveTemplateIdentifierAsync(
            settings.TemplateIdentifier,
            promptsAllowed,
            cancellationToken
        );
        if (!templateIdentifierResult.IsSuccess)
        {
            _terminalSession.WriteErrorLine(templateIdentifierResult.ErrorMessage!);
            return templateIdentifierResult.ExitCode;
        }

        CreateWorkspaceCommand command = new(workspaceNameResult.Value!, templateIdentifierResult.TemplateIdentifier!);
        CreateWorkspaceCommandResult result = await _createWorkspaceCommandHandler.HandleAsync(
            command,
            cancellationToken
        );
        if (!result.IsSuccess)
        {
            _terminalSession.WriteErrorLine(result.Failure!.Message);
            return result.Failure.ExitCode;
        }

        CreatedWorkspace createdWorkspace = result.Workspace!;
        _terminalSession.WriteLine(
            $"Created workspace '{createdWorkspace.WorkspaceName}' from template '{createdWorkspace.TemplateIdentifier}' ({templateIdentifierResult.SelectionSource})."
        );
        return ExitCodes.Success;
    }

    private async Task<WorkspaceNameResolutionResult> ResolveWorkspaceNameAsync(
        string? workspaceName,
        bool promptsAllowed,
        CancellationToken cancellationToken
    )
    {
        if (!string.IsNullOrWhiteSpace(workspaceName))
            return WorkspaceNameResolutionResult.Success(workspaceName.Trim());

        if (!promptsAllowed)
        {
            string message = _terminalSession.IsInteractive
                ? "Workspace name is required when interactive prompts are disabled."
                : "Workspace name is required when the terminal is not interactive.";
            return WorkspaceNameResolutionResult.Failure(message);
        }

        TerminalTextPrompt prompt = new("Workspace name:", "Workspace name is required.");
        TerminalTextInputResult promptResult = await _terminalSession.PromptForTextAsync(prompt, cancellationToken);
        if (promptResult.WasCancelled || string.IsNullOrWhiteSpace(promptResult.Value))
            return WorkspaceNameResolutionResult.Cancelled();

        return WorkspaceNameResolutionResult.Success(promptResult.Value);
    }

    private async Task<TemplateIdentifierResolutionResult> ResolveTemplateIdentifierAsync(
        string? templateIdentifier,
        bool promptsAllowed,
        CancellationToken cancellationToken
    )
    {
        if (!string.IsNullOrWhiteSpace(templateIdentifier))
            return TemplateIdentifierResolutionResult.Explicit(templateIdentifier.Trim());

        if (!promptsAllowed)
        {
            string message = _terminalSession.IsInteractive
                ? "Template selection requires `--template <identifier>` when interactive prompts are disabled."
                : "Template selection requires `--template <identifier>` when the terminal is not interactive.";
            return TemplateIdentifierResolutionResult.Failure(message);
        }

        ListTemplatesQueryResult queryResult = await _listTemplatesQueryHandler.HandleAsync(
            new ListTemplatesQuery(),
            cancellationToken
        );
        if (!queryResult.IsSuccess)
            return TemplateIdentifierResolutionResult.Failure(
                queryResult.Failure!.Message,
                queryResult.Failure.ExitCode
            );

        IReadOnlyList<ListedTemplate> templates = queryResult.Templates!;
        if (templates.Count == 0)
            return TemplateIdentifierResolutionResult.Failure(
                "No templates are available. Run `nfw templates` after restoring a catalog source.",
                ExitCodes.RuntimeFailure
            );

        TerminalSelectionOption[] options = templates
            .Select(t => new TerminalSelectionOption(
                t.Identifier,
                $"{t.Identifier} | {t.DisplayName} | {t.Description}"
            ))
            .ToArray();

        TerminalSelectionPrompt prompt = new("Select a template", options);
        TerminalSelectionResult promptResult = await _terminalSession.PromptForSelectionAsync(
            prompt,
            cancellationToken
        );
        if (promptResult.WasCancelled || string.IsNullOrWhiteSpace(promptResult.SelectedValue))
            return TemplateIdentifierResolutionResult.Cancelled();

        return TemplateIdentifierResolutionResult.Interactive(promptResult.SelectedValue!);
    }

    private sealed class WorkspaceNameResolutionResult
    {
        private WorkspaceNameResolutionResult(string? value, string? errorMessage, int exitCode)
        {
            Value = value;
            ErrorMessage = errorMessage;
            ExitCode = exitCode;
        }

        public string? Value { get; }

        public string? ErrorMessage { get; }

        public int ExitCode { get; }

        public bool IsSuccess => Value is not null;

        public static WorkspaceNameResolutionResult Success(string workspaceName)
        {
            return new WorkspaceNameResolutionResult(workspaceName, null, ExitCodes.Success);
        }

        public static WorkspaceNameResolutionResult Failure(string errorMessage)
        {
            return new WorkspaceNameResolutionResult(null, errorMessage, ExitCodes.UsageError);
        }

        public static WorkspaceNameResolutionResult Cancelled()
        {
            return new WorkspaceNameResolutionResult(null, "Workspace creation was cancelled.", ExitCodes.Interrupted);
        }
    }

    private sealed class TemplateIdentifierResolutionResult
    {
        private TemplateIdentifierResolutionResult(
            string? templateIdentifier,
            string? selectionSource,
            string? errorMessage,
            int exitCode
        )
        {
            TemplateIdentifier = templateIdentifier;
            SelectionSource = selectionSource;
            ErrorMessage = errorMessage;
            ExitCode = exitCode;
        }

        public string? TemplateIdentifier { get; }

        public string? SelectionSource { get; }

        public string? ErrorMessage { get; }

        public int ExitCode { get; }

        public bool IsSuccess => TemplateIdentifier is not null;

        public static TemplateIdentifierResolutionResult Explicit(string templateIdentifier)
        {
            return new TemplateIdentifierResolutionResult(templateIdentifier, "explicit", null, ExitCodes.Success);
        }

        public static TemplateIdentifierResolutionResult Interactive(string templateIdentifier)
        {
            return new TemplateIdentifierResolutionResult(templateIdentifier, "interactive", null, ExitCodes.Success);
        }

        public static TemplateIdentifierResolutionResult Failure(
            string errorMessage,
            int exitCode = ExitCodes.UsageError
        )
        {
            return new TemplateIdentifierResolutionResult(null, null, errorMessage, exitCode);
        }

        public static TemplateIdentifierResolutionResult Cancelled()
        {
            return new TemplateIdentifierResolutionResult(
                null,
                null,
                "Workspace creation was cancelled.",
                ExitCodes.Interrupted
            );
        }
    }
}
