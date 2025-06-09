# Tester Service (`apps/tester`)

## Runtime
* **Node**: 18
* **Package Manager**: pnpm workspace

## Docker Image
This service uses the same Playwright container as the crawler:
`mcr.microsoft.com/playwright:v1.44`

## Run
Launch the service with:
```bash
HEADLESS=1 pnpm start
```
It expects the `SERVICE_URL` environment variable to be set to the target API endpoint.

## Tests
All tests must complete in under 15 seconds, or they will be marked as a failure. This is a strict performance requirement. 