# Scorer Service (`apps/scorer`)

## Language and Module
* **Language**: Go 1.22
* **Module Path**: `github.com/larptech/scorer`

## Build
```bash
go build ./cmd/scorer
```

## Test
```bash
go test ./...
```
- Integration tests require a Redis instance. They are spun up via `docker run redis:7` automatically.
- Test coverage must be ≥ 85%. 