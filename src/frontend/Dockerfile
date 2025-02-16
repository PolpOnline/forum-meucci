# Use an official Node.js image as a parent image
FROM node:alpine AS builder

# Install pnpm
RUN npm install -g pnpm

# Set the working directory
WORKDIR /app

# Copy the package.json and pnpm-lock.yaml files
COPY package.json pnpm-lock.yaml ./

# Install dependencies
RUN pnpm install

# Copy the rest of the application code
COPY . .

# Build the SvelteKit project
RUN pnpm build

# Use a minimal base image for the final stage
FROM node:alpine

# Set the working directory
WORKDIR /app

# Copy the built files from the builder stage
COPY --from=builder /app/build /app/build

# Install pnpm in the final stage
RUN npm install -g pnpm

# Copy the package.json and pnpm-lock.yaml files
COPY package.json pnpm-lock.yaml ./

# Install only production dependencies
RUN pnpm install --prod

# Expose the port the app runs on
EXPOSE 3000

# Set the API endpoint and run the SvelteKit app
# (temporary solution till coolify supports --add-host, https://github.com/coollabsio/coolify/discussions/5136)
ENTRYPOINT ["/bin/sh", "-c" , "echo 172.17.0.1 forum-api.meucci.party >> /etc/hosts && exec node build" ]