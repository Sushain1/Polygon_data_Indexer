 Polygon Data Indexer

A Rust-based application for indexing and storing blockchain data from the Polygon network. This project uses the `sqlx` crate for asynchronous database operations with a SQLite backend, providing a fast and lightweight solution for data retrieval and storage.
 
Features
-   Connects to the Polygon blockchain.
-   Fetches and processes transaction data.
-   Stores indexed data in a local SQLite database.
-   Built with Rust for high performance and reliability.

 Prerequisites
To get this project up and running, you need the following software installed on your machine:

* **Rust & Cargo:** The Rust programming language and its package manager.
* **Git:** A version control system to clone the repository.
* **PowerShell (Windows):** For running commands that manage environment variables.


 Setup and Installation
Follow these steps to set up and run the project:

1.  **Clone the repository:**
    ```bash
    git clone [https://github.com/Sushain1/Polygon_data_Indexer](https://github.com/Sushain1/Polygon_data_Indexer)
    cd polygon_data_indexer
    ```

2.  **Configure Environment Variables:**
    Create a file named `.env` in the root of your project. This file stores sensitive information, like database URLs, and is ignored by Git to keep it secure.
    Add the following line to your `.env` file:
    ```
    DATABASE_URL=sqlite:polygon_indexer.db
    ```

3.  **Prepare the Database Query Cache:**
    The `sqlx` crate performs compile-time checks on your SQL queries. To do this, it needs to connect to the database to verify the schema. We'll use a specific command to pass the database URL and generate a local cache.

    In your PowerShell terminal, run:
    ```bash
    $env:DATABASE_URL="sqlite:polygon_indexer.db" ; cargo sqlx prepare
    ```
    This will create a `.sqlx` directory containing the query cache. You should commit this folder to version control to allow others to compile the project easily.

4.  **Run the application:**
    Since the project contains multiple executable binaries (`polygon_data_indexer` and `query_tool`), you must specify which one to run.

    To run the main data indexer:
    ```bash
    cargo run --bin polygon_data_indexer
    ```

## Project Structure

* **`src/main.rs`**: The main entry point of the application.
* **`src/db.rs`**: Contains the logic for database operations, including the `sqlx` queries.
* **`Cargo.toml`**: The manifest file for your project, which lists dependencies and project metadata.
* **`.env`**: A local file for environment variables like `DATABASE_URL`. It is ignored by Git.
* **`.gitignore`**: Specifies files and folders that Git should not track, such as `target/` and the database files.
* **`.sqlx`**: The generated `sqlx` query cache used for compile-time SQL validation.

## Important Notes

* **Line Endings (LF/CRLF):** If you see warnings about line endings during `git add`, don't worry. This is a normal and harmless difference between Windows and other operating systems.
* **SQLx and Caching:** The `sqlx` crate's compile-time checks are a powerful feature. The `cargo sqlx prepare` command makes this process offline, meaning you won't need an active database connection every time you compile the code.
