# ECS Fargate Infrastructure

This directory contains the Terraform configuration for deploying the services to AWS Fargate.

## Task Definitions
Each service has a corresponding `task-definition.json` file. These are generated from the `docker-compose.yaml` file using a tool like `container-transform`.

## Networking
- The `api` service is exposed to the public internet via an Application Load Balancer (ALB) target group.
- All other services (`listener`, `metadata`, `crawler`, `scorer`) are behind a private Network Load Balancer (NLB) and are not publicly accessible.

## Auto-Scaling
- Services are configured to auto-scale based on CPU utilization.
- A new task is provisioned when CPU utilization exceeds 70%.
- Each service maintains a minimum of 1 running task and can scale up to a maximum of 10 tasks. git init