build-spring-image:
	cd spring && ./mvnw package && docker build -t spring-example-app:latest .
compose-up:
	docker-compose up -d
docker-stop-container:
	docker stop $(docker ps -q)
docker-remove-container:
	docker rm $(docker ps -aq)