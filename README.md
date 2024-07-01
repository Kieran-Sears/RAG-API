used in conjunction with [open_llama_3b-f16.bin](https://huggingface.co/rustformers/open-llama-ggml/tree/main) for testing to see if [rustformers](https://github.com/rustformers/llm) works out the box.


# Development

## docker compose

If using WSL2 ensure that the docker desktop app is already running in windows for it to be accessible via linux.

`docker compose up -d` will start all services in a single command.

`docker compose down -v`  will stop containers and remove any volumes of data that was created by the docker compose up -d command.

### Database
Using postgres db with adminer which you can open on the port found in the docker-compose.yaml file under the services section called "adminer", this also includes some other authorization values needed to log into adminer.

When spun up the database will create tables in accordance to files found under `resources/migrations/<timestamp>_create_posts`, loading them in order of the timestamp value that is included in each folder name.

