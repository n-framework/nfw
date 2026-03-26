SHELL := bash

.PHONY: build test format analyze build-packages test-packages build-all test-all

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

build-packages:
	$(MAKE) -C packages/n-framework-core-cli build
	$(MAKE) -C packages/n-framework-core-template build

test-packages:
	$(MAKE) -C packages/n-framework-core-cli test
	$(MAKE) -C packages/n-framework-core-template test

build-all: build build-packages

test-all: test test-packages
