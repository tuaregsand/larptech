# API Service (`apps/api`)

## Language and Frameworks
* **Language**: Go 1.22
* **Frameworks**: Fiber (web framework) + gqlgen (GraphQL server)

## Build & Run (Development)
```bash
go run main.go
```

## GraphQL Playground
The GraphQL playground is available at the root path (`/`) of the service.

## Query Examples
```graphql
# Example: Get tokens by color
query GetRedTokens {
  tokens(color: "red") {
    id
    color
    score
  }
}
``` 