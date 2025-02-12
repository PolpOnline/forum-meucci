<div align="center">
  <h1>Forum Meucci</h1>
</div>

## Installation and usage

Use the dockerfiles
to build the frontend and backend images and run them with Docker.

Note that the project requires configuration in `.env`,
see the example file in the `src/backend` directory.

### Backend envs

The backend requires the following environment variables to be set:

- `DATABASE_URL` - a Postgres database URL
- `COOKIE_KEY` - a 64-byte key to encrypt cookies (see below for instructions on how to generate one)
- `REDIS_URL` - a Redis database URL
- `SITE_URL` - the URL of the frontend site
- `GOOGLE_CLIENT_ID` - the Google OAuth client ID
- `GOOGLE_CLIENT_SECRET` - the Google OAuth client secret
- `BACKEND_URL` - the URL of the backend site
- `EMAIL_DOMAIN` - the domain to allow users to sign up with
- `COOKIE_DOMAIN` - the domain to set the session cookie on (usually the same as `SITE_URL`, with the protocol and the
  subdomain removed)

#### Generate a cookie key

To generate a cookie key,
you need to spin up a new Rust project with `cargo new your_project_name`
and paste the following code:

```toml
# Cargo.toml
rand = "0.9.0-alpha.1"
axum-extra = { version = "0.9.3", features = ["cookie-private"] }
```

```rust
// src/main.rs
use axum_extra::extract::cookie::Key;
use rand::{Rng, thread_rng};

fn main() {
    // Generate a cryptographically random key of 64 bytes
    let mut rng = thread_rng();
    let mut random_key = [0u8; 64];
    rng.fill(&mut random_key);
    match Key::try_from(&random_key[..]) {
        Ok(key) => {
            println!("Random key: {:?}", key.master());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
```

Then run the project with cargo run and copy the cookie key to the .env file in the backend directory.

Cd into the both frontend and back end directories and run the following commands:

```bash
cd src/frontend
docker build -t forum-meucci-frontend .
cd ../backend
docker build -t forum-meucci-backend .
```

Then run the following command to start the containers:

```bash
docker run -d -p 5173:5173 forum-meucci-frontend
docker run -d -p 3000:3000 --env-file .env forum-meucci-backend
```

Remember to set the environment variables
while deploying to prod as `.env` won't be copied over to the Docker container.

## Development

To run the frontend and backend in development mode, you can use the following commands:

```bash
cd src/frontend
pnpm install
pnpm dev
```

```bash
cd src/backend
cargo run
```

## License

This project is licensed under the MIT License, see the [LICENSE](LICENSE) file for details.
