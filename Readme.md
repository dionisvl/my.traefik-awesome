# Traefik Multi-Environment Setup
This project demonstrates a multi-environment Docker setup using Traefik as a reverse proxy,  
with separate configurations for development and production environments.

## Project Structure
```
.
├── app1/
│   ├── Dockerfile
│   ├── go.mod
│   └── main.go
├── app2/
│   ├── Dockerfile
│   ├── go.mod
│   └── main.go
├── compose.yaml
├── compose.override.dev.yaml
├── compose.override.prod.yaml
├── .env
├── .env.prod
└── Makefile
```

## Features

* Separate configurations for development and production environments
* Automatic HTTPS with Let's Encrypt in production 
* Local HTTP setup for development 
* Traefik dashboard with authentication in production 
* Multiple App services behind Traefik

## Installation
### Dev environment
- add to hosts: 
```
127.0.0.1 app1.local
127.0.0.1 app2.local
127.0.0.1 traefik.localhost
```
- `cp .env.example .env` `cp .env.prod.example .env.prod` for prod environment
- `cp compose.override.dev.yaml compose.override.yaml`
- `make up`
### Prod environment
- `cp .env.prod.example .env.prod`
- `cp compose.override.prod.yaml compose.override.yaml`
- admin password generation
  - `sudo apt-get install apache2-utils`
  - `htpasswd -nb admin your_password` or `htpasswd -nb admin your_password | sed -e s/\\$/\\$\\$/g` (compose compatible)
- `make up`
