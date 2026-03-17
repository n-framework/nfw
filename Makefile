SHELL := bash

.PHONY: build test format analyze

build:
	dotnet build NFramework.Nfw.slnx

test:
	dotnet test NFramework.Nfw.slnx

format:
	dotnet tool restore
	dotnet csharpier .

analyze:
	dotnet tool restore
	dotnet roslynator analyze
