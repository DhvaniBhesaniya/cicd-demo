# Rust + Axum + Tokio — Docker & CI/CD on Render

A beginner-friendly project that teaches you:
- Building a REST API with **Rust**, **Axum**, and **Tokio**
- Containerising it with **Docker** (multi-stage build)
- Automating tests + deployment with **GitHub Actions** (CI/CD)
- Deploying for free on **Render**

---

## Project Structure

```
docker-cicd-demo/
├── src/
│   └── main.rs              ← All API code (routes, handlers, server)
├── Cargo.toml               ← Rust dependencies
├── Dockerfile               ← Multi-stage Docker build
├── docker-compose.yml       ← Run locally with one command
├── render.yaml              ← Render deployment config
└── .github/
    └── workflows/
        └── ci-cd.yml        ← GitHub Actions pipeline
```

---

## 1 — The API (Axum + Tokio)

### What is Tokio?
Tokio is an **async runtime** for Rust. When you write `async fn` in Rust,
you need a runtime to actually execute those futures. Tokio does that — it
manages threads and schedules async tasks efficiently.

```rust
#[tokio::main]   // ← this macro starts the Tokio runtime
async fn main() {
    // now you can use .await anywhere here
}
```

### What is Axum?
Axum is a web framework built on top of Tokio. You define routes and handlers:

```rust
Router::new()
    .route("/todos", get(list_todos).post(create_todo))
```

Each handler is an `async fn` that returns something Axum knows how to
serialize (JSON, status codes, etc).

### Run locally (without Docker)
```bash
cargo run
# Visit http://localhost:3000
```

---

## 2 — Docker (Multi-stage Build)

### Why Docker?
Docker packages your app + all its dependencies into a portable **image**.
"Works on my machine" becomes "works everywhere" — your laptop, CI, Render.

### Why multi-stage?
Rust's compiler and toolchain are large (~1 GB). We don't need them at runtime.
Multi-stage builds let us:
1. **Stage 1 (builder):** compile the binary using the full Rust image
2. **Stage 2 (runtime):** copy only the binary into a tiny Debian image (~20 MB)

```
┌──────────────────────┐      ┌─────────────────────┐
│  Stage 1: builder    │      │  Stage 2: runtime    │
│  rust:1.78-slim      │─────▶│  debian:bookworm-slim│
│  + Cargo.toml        │ copy │  + binary only       │
│  + src/              │ bin  │  ~20 MB total        │
│  → compiles binary   │      │                      │
└──────────────────────┘      └─────────────────────┘
```

### Layer caching trick
We copy `Cargo.toml` and build a dummy `main.rs` **before** copying real source.
This means Docker caches the compiled dependencies. Next build only recompiles
your code — saving minutes on each CI run.

```dockerfile
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release   # ← cached unless Cargo.toml changes
COPY src ./src
RUN cargo build --release   # ← fast: only recompiles your code
```

### Commands
```bash
# Build the image
docker build -t docker-cicd-demo .

# Run it
docker run -p 3000:3000 docker-cicd-demo

# Or use docker-compose (easier)
docker compose up --build
```

---

## 3 — CI/CD Pipeline (GitHub Actions)

### What is CI/CD?
- **CI (Continuous Integration):** automatically test every code change
- **CD (Continuous Deployment):** automatically deploy when tests pass

### How the pipeline works

```
Push to main / Open PR
        │
        ▼
┌─────────────┐    ┌─────────────┐
│  test       │    │  lint       │   ← Run in parallel on every push/PR
│  cargo test │    │  clippy     │
└──────┬──────┘    └──────┬──────┘
       └─────────┬────────┘
                 │ both must pass
                 ▼
        ┌────────────────┐
        │  docker        │   ← Only on merge to main
        │  build & push  │
        │  to Docker Hub │
        └───────┬────────┘
                │
                ▼
        ┌────────────────┐
        │  deploy        │   ← Hits Render's deploy webhook
        │  Render redeploys│
        └────────────────┘
```

### Setting up GitHub Secrets
Go to your repo → Settings → Secrets and variables → Actions → New secret:

| Secret name           | Value                                  |
|-----------------------|----------------------------------------|
| `DOCKERHUB_USERNAME`  | Your Docker Hub username               |
| `DOCKERHUB_TOKEN`     | Docker Hub access token (not password) |
| `RENDER_DEPLOY_HOOK`  | URL from Render → Settings → Deploy Hook|

---

## 4 — Deploy to Render (Free)

1. Push this repo to GitHub
2. Go to [render.com](https://render.com) → New → Web Service
3. Connect your GitHub repo
4. Render detects `render.yaml` automatically and configures everything
5. Click **Deploy**

Render will:
- Pull your code
- Build the Docker image
- Run it and expose it on a public HTTPS URL

After the first manual deploy, every push to `main` triggers automatic
redeployment via the GitHub Actions pipeline.

> **Free tier note:** Render's free plan spins the service down after
> 15 minutes of inactivity. First request after sleep takes ~30 seconds.
> Upgrade to starter ($7/mo) for always-on.

---

## 5 — API Endpoints

| Method | Path        | Description          |
|--------|-------------|----------------------|
| GET    | `/`         | Welcome message      |
| GET    | `/health`   | Health check         |
| GET    | `/todos`    | List all todos       |
| GET    | `/todos/:id`| Get a single todo    |
| POST   | `/todos`    | Create a todo        |

### Example requests
```bash
# Health check
curl http://localhost:3000/health
curl https://cicd-demo-rgew.onrender.com/health

# List todos
curl http://localhost:3000/todos
curl https://cicd-demo-rgew.onrender.com/todos

# Create a todo
curl -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Deploy to production"}'


  curl -X POST https://cicd-demo-rgew.onrender.com/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Deploy to production"}'
```

---
### Live requests
```bash
# Health check
curl https://cicd-demo-rgew.onrender.com/health

# List todos
curl https://cicd-demo-rgew.onrender.com/todos

# Create a todo
  curl -X POST https://cicd-demo-rgew.onrender.com/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Deploy to production"}'
```
---

## Learning Path (Next Steps)

Once you understand this project, try adding:
1. **PostgreSQL** with `sqlx` — replace the static data with a real DB
2. **Authentication** with JWT using `axum-extra`
3. **Integration tests** with `axum::Server` in test mode
4. **Environment config** with the `dotenvy` crate
5. **Metrics** with `prometheus` + `axum-prometheus`