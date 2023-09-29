# Use an official Rust base image
FROM rust:1.70.0

# Set the working directory inside the container
WORKDIR /usr/src/myapp

# Copy the entire project into the container
COPY . .

# Build the application
RUN cargo build

# This will run your tests when the container starts. If you'd rather build and test in one step, you can chain commands using &&
CMD ["cargo", "test"]