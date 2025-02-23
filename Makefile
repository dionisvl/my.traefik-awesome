up:
	docker compose down -t 0 && docker compose up --build

down:
	docker compose down -t 0 --remove-orphans

sh:
	docker compose exec app-php-laravel sh
