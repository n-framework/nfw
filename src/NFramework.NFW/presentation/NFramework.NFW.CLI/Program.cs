using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using NFramework.NFW.CLI;
using NFramework.NFW.CLI.Startup;

IHostBuilder builder = Host.CreateDefaultBuilder(args);
builder.ConfigureServices(
    (context, services) =>
    {
        ParsedArguments parsedArguments = ParsedArguments.Parse(args);

        // Create CLI services with the parsed arguments
        CliServices cliServices = CliServiceCollectionFactory.Create(parsedArguments);

        // Copy all services from the CLI service collection
        foreach (ServiceDescriptor service in cliServices.Services)
        {
            services.Add(service);
        }
    }
);

using IHost host = builder.Build();
NfwCliApplicationOrchestrator orchestrator = host.Services.GetRequiredService<NfwCliApplicationOrchestrator>();
return orchestrator.Run(args);
