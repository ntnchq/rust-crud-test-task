DOCKER_COMPOSE=docker-compose
MIGRATION_NAME?=migration_name

up:
	$(DOCKER_COMPOSE) up -d

down:
	$(DOCKER_COMPOSE) down

logs:
	$(DOCKER_COMPOSE) logs -f app

mig-diff:
	diesel migration generate $(MIGRATION_NAME)

mig-apply:
	diesel migration run