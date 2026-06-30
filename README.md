### Actix-web starter

**REST API for modern web applications**

*Written on [Actix Web](https://actix.rs/) web framework*

#### Implemented features:

- Route handlers
- SQLx based connection to **PostgresDB**
- Request payload validation
- **Session** based authentication
- Caching middleware based on **Redis**
- Monitoring with **Prometheus**
- Logging with **env_logger**

#### How to use

- make sure [Rust](https://rust-lang.org/) is installed
- clone the repo
- set up environment variables. *copy .env.example pattern* to *.env* and edit the variables 
- `cargo build` -- *build a binary*
- `cargo run` -- *run the server*

---

**Notes:**

Optional custom caching middleware once you need more control on invalidation *libs/middleware/redis.rs*

Crates you might want to use should you decide to develop further on Rust *Cargo.toml* 
