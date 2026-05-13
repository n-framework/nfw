# Research: Gen Endpoint Command

## Needs Clarification Resolution

All technical details are well known. No required clarification.

## Best Practices

- Standard Minimal API configurations using `Map{Verb}`.
- Include typical OpenAPI descriptors like `.WithName(...)`, `.WithTags(...)`, and `.Produces<T>(...)`.
- Ensure mapping between MediatR request and HTTP endpoint aligns cleanly.
