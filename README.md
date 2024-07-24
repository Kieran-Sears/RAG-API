
# QuizMaster

## Overview

### Objective:
Generate a map of topics that branch out into questions which branch out into answers, such that quizzes may be
created from it to test a users knowledge in a given domain previously asked about by the user in ChatGPT.

### Prerequisites:
1. a table of conversations.
2. a table of exchanges.
3. a table of questions.
4. a table of answers.
5. a table of topics.
6. a table of refined questions.
7. a table of refined answers.

### Import Playthrough
split datafile into conversations
cycle through conversations
    cycle through mappings
        cycle through author_metadata
            cycle through metadata
                upsert metadata
            upsert author_metadata
        upsert mapping
    upsert conversation

### Assumptions To Avoid:
conversations always happen in chronological order
conversations are always on topic, unrelated questions aren't included randomly
conversations don't drift and then get back on topic
all questions are going to be closely related
users are going to give feedback on whether answers were accurate / successful
first answers are going to not going to need additional refinement

### Scrubbing Playthrough
cycle through conversations
    cycle through mappings
        isolate user exchanges
            isolate questions (initial questions vs answer follow-up questions)
                group questions by overarching topics
            isolate corrections (statements that reflect dissatisfaction with a given answer)
        isolate system exchanges
            isolate parents
                marry to initial questions (recursively lookup through follow-up questions)


## Technology

Models: [open_llama_3b-f16.bin](https://huggingface.co/rustformers/open-llama-ggml/tree/main)
Database: PostgreSQL
Database Connector: Diesel
Async Backend: Tokio
Web API: Axum
LLM plugin: [rustformers](https://github.com/rustformers/llm)

## Developer Notes

### docker compose

If using WSL2 ensure that the docker desktop app is already running in windows for it to be accessible via linux.

`docker compose up -d` will start all services in a single command.

`docker compose down -v`  will stop containers and remove any volumes of data that was created by the docker compose up -d command.

### Database
Using postgres db with adminer which you can open on the port found in the docker-compose.yaml file under the services section called "adminer", this also includes some other authorization values needed to log into adminer.

When spun up the database will create tables in accordance to files found under `resources/migrations/<timestamp>_create_posts`, loading them in order of the timestamp value that is included in each folder name.


## Glossary

- Answer: response given by ChatGPT to a question asked. 

- Conversation: "chat" in ChatGPT which is given a topic title and contains multiple exchanges.

- Exchange: single unit of a conversation, either something the user sent to ChatGPT, or a response given by ChatGPT.

- Question: user given question in an exchange within a conversation.
