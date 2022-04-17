# Rust as the base image
FROM rust:1.59 as build

# Create a new empty shell project
RUN USER=root cargo new --bin yew-markdown 
WORKDIR /yew-markdown

# Copy all the files
COPY . .

RUN cargo install trunk wasm-bindgen-cli
# RUN trunk build --release
CMD trunk serve --release

