namespace NFramework.NFW.Application.Features.Cli.Configuration;

public readonly record struct Result<T>
{
    public bool IsSuccess { get; }
    public bool IsFailure => !IsSuccess;
    public T? Value { get; }
    public string? Error { get; }
    public Exception? Exception { get; }

    private Result(bool isSuccess, T? value, string? error, Exception? exception)
    {
        IsSuccess = isSuccess;
        Value = value;
        Error = error;
        Exception = exception;
    }

    public static Result<T> Success(T value) => new(true, value, null, null);

    public static Result<T> Failure(string error, Exception? exception = null) => new(false, default, error, exception);

    public TResult Match<TResult>(Func<T, TResult> onSuccess, Func<string, Exception?, TResult> onFailure)
    {
        return IsSuccess ? onSuccess(Value!) : onFailure(Error ?? "Unknown error", Exception);
    }
}
