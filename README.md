# Vaultash

## Overview
Vaultash is a web application designed to manage and store service credentials securely. It provides RESTful API endpoints for creating, retrieving, updating, and deleting vault entries, which include service names, usernames, passwords, and emails.

## Features
- **Password Generation**: Automatically generate strong passwords for services.
- **CRUD Operations**: Perform create, read, update, and delete operations on vault entries.
- **Database Integration**: Connects to a database using SeaORM for data persistence.
- **CORS Configuration**: Configurable CORS policy to control access from different origins.

## Project Structure
- **`src/main.rs`**: Entry point of the application. Initializes the database, applies migrations, and sets up the web server.
- **`src/api/`**: Contains API handlers for vault operations.
  - **`handlers/vault_handler.rs`**: Implements the logic for CRUD operations on vault entries.
- **`src/db/`**: Database connection and initialization logic.
  - **`conn.rs`**: Manages database connection using SeaORM.
- **`src/models/`**: Defines data models and DTOs.
  - **`entities/vault.rs`**: Entity model for the vault table.
  - **`dtos/vault.rs`**: Data Transfer Objects for vault operations.
- **`src/utils/`**: Utility functions and configurations.
  - **`index.rs`**: Contains CORS layer configuration and password generation logic.

## Getting Started
### Prerequisites
- Rust and Cargo installed
- A running instance of a compatible database
- `.env` file with `DATABASE_URL` configured

### Running the Application
1. **Initialize the Database**: Ensure the database is running and accessible.
2. **Run Migrations**: Automatically handled during application startup.
3. **Start the Server**: Use `cargo run` to start the server on `http://localhost:5000`.

## API Endpoints
- **GET /api/admin/vaults**: Retrieve all vault entries.
- **POST /api/admin/vaults**: Create a new vault entry.
- **GET /api/admin/vaults/{id}**: Retrieve a vault entry by ID.
- **PUT /api/admin/vaults/{id}**: Update a vault entry by ID.
- **DELETE /api/admin/vaults/{id}**: Delete a vault entry by ID.

## Security Considerations
- **CORS**: Configured to allow specific origins. Modify `cors_layer` in `utils/index.rs` for production.
- **Environment Variables**: Ensure sensitive information like `DATABASE_URL` is stored securely.

## License
This project is licensed under the MIT License.
