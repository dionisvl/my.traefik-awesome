# Traefik Multi-Environment Setup
This project demonstrates a multi-environment Docker setup using Traefik as a reverse proxy,  
with separate configurations for development and production environments.

## Project Structure
```
.
├── app1/                      # Go application
│   ├── Dockerfile
│   ├── go.mod
│   └── main.go
├── app2/                      # PHP application
│   ├── Dockerfile
│   ├── nginx.conf            # Nginx configuration for PHP
│   ├── supervisord.conf      # Supervisor config for PHP-FPM + Nginx
│   └── index.php
├── app3/                      # Static HTML application
│   ├── Dockerfile
│   ├── nginx.conf            # Nginx configuration for static files
│   └── index.html
├── compose.yaml              # Base compose configuration
├── compose.override.dev.yaml # Development environment overrides
├── compose.override.prod.yaml # Production environment overrides
├── .env.example             # Example development environment variables
├── .env.prod.example        # Example production environment variables
└── Makefile                 # Build and deployment commands
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
