# user_register_api
A simple RESTful API built with Rust, utilizing the Rocket framework and Diesel ORM for PostgreSQL database interactions. 🦀

## Features

- User registration with secure password hashing
- CRUD operations for user management
- PostgreSQL integration using Diesel ORM
- Environment configuration via `.env`
- Connection pooling with `r2d2`
- JSON serialization/deserialization with `serde`

## Getting Started

### Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Diesel CLI

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/filipe-freitas-dev/user_register_api.git
   cd user_register_api
   ```

2. **Set up environment variables:**
   Create a `.env` file in the root directory and add your database URL:
   ```env
   DATABASE_URL=postgres://username:password@localhost/database_name
   ```

3. **Install Diesel CLI (if not already installed):**
   ```bash
   cargo install diesel_cli --no-default-features --features postgres
   ```

4. **Run database migrations:**
   ```bash
   diesel setup
   diesel migration run
   ```

5. **Build and run the application:**
   ```bash
   cargo run
   ```

## API Endpoints

| Method | Endpoint       | Description          |
|--------|----------------|----------------------|
| POST   | `/users`       | Create a new user    |
| GET    | `/users`       | Retrieve all users   |
| GET    | `/users/{id}`  | Retrieve user by ID  |
| PUT    | `/users/{id}`  | Update user by ID    |
| DELETE | `/users/{id}`  | Delete user by ID    |

## Technologies Used

- [Rust](https://www.rust-lang.org/)
- [Rocket](https://rocket.rs/)
- [Diesel](https://diesel.rs/)
- [PostgreSQL](https://www.postgresql.org/)
- [Serde](https://serde.rs/)
- [dotenvy](https://crates.io/crates/dotenvy)
- [r2d2](https://crates.io/crates/r2d2)

## License

This project is licensed under the [MIT License](LICENSE).
