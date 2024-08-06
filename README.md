# Task-Manager

This is a simple command-line task manager application built with Rust and Diesel.

## Prerequisites

- Rust (https://www.rust-lang.org/)
- Cargo (included with Rust)
- PostgreSQL (https://www.postgresql.org/)
- Diesel (https://diesel.rs/)

## Installation

1. *Clone the repository:*
   
   git clone https://github.com/your-username/task-manager.git
   
2. *Navigate to the project directory:*

   cd task-manager
   
3. *Install dependencies:*
   
   cargo build
   
4. *Set up the database:*
   - Create a PostgreSQL database.
   - Create a `.env` file in the project root with the following environment variable:
     
     DATABASE_URL=postgres://user:password@localhost/your_database_name
     
     Replace `user`, `password`, and `your_database_name` with your actual database credentials.
5. *Run migrations:*
   - Run the following command to set up Diesel migrations:

     diesel setup
     
   - This will create a `migrations` folder.
   - Open the `migrations` folder and find the latest migration folder.
   - Edit the `up.sql` file inside the migration folder with the following SQL statement:
     
     CREATE TABLE tasks (
         id SERIAL PRIMARY KEY,
         name VARCHAR(255) NOT NULL
     );
     
   - Edit the `down.sql` file with the following SQL statement:
     
     DROP TABLE tasks;
     
   - Run the following command to apply the migrations:
     
     diesel migration run
     

## Usage

   cargo run
