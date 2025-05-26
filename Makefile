build_image:
	docker buildx build --platform linux/amd64,linux/arm64,linux/arm/v7 -t prongbang/notify:latest -f deployments/Dockerfile .

push_image:
	docker tag prongbang/notify:latest prongbang/notify:1.0.1
	docker image push prongbang/notify:latest
	docker image push prongbang/notify:1.0.1

build_push_image:
	make build_image
	make push_image

run:
	docker run \
	-e SERVER_HOST="0.0.0.0" \
	-e BUDDHA_ENDPOINT="http://buddha.com" \
	-e DISCORD_WEBHOOK_URL="http://discord.com" \
	-e API_KEY="XYZ" \
	-it -p 9001:9001 prongbang/notify:latest
