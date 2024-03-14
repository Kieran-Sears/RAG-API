FROM rust:1.76

WORKDIR /
COPY . .
COPY C:/Users/catri/Kieran/Personal/Projects/Hugging Face Models/Magicoder-S-DS-6.7B-GPTQ/ ./model/

RUN cargo install --path .

CMD ["/src/main.rs"]