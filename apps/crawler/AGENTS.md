# Crawler Service (`apps/crawler`)

## Runtime
* **Node**: 18
* **Package Manager**: pnpm workspace

## Build / Dependencies
```bash
pnpm i --frozen-lockfile
```

## Test
```bash
pnpm test
```

## Docker Image
This service uses a pinned Playwright docker base image for consistency.
`mcr.microsoft.com/playwright:v1.44`

## Configuration
A `patterns.yaml` file is required to provide regex patterns for claims to be extracted. 