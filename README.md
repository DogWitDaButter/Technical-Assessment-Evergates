# Evergates Backend Development

## Project Structure

- `src/main.rs`: The main entry point of the application.
- `.env`: Environment variables for database configuration. (You should create this file yourself and further clear instructions will be provided)
- `Cargo.toml`: Project dependencies.

## Setup Instructions

### Prerequisites

- Rust and Cargo installed. You can install them from [here](https://www.rust-lang.org/tools/install).
- PostgreSQL installed and running. You can get it from [here](https://www.postgresql.org/download/)

### Steps

1. Clone the repository:

   ```sh
   git clone https://github.com/DogWitDaButter/Technical-Assessment-Evergates.git
   cd Technical-Assessment-Evergates/evergates
   ```
2. Prepare the Database:

    Ensure your PostgreSQL server is running, and then create the necessary database and table.
    
    Using the command line run :

    ```sh
    # Access PostgreSQL with your username (replace 'your_username' with your actual username)

    psql -U your_username
    ```
    Inside the PostgreSQL shell, run :

    ```sh
    #Create a new database named 'my_db'

    CREATE DATABASE my_db; 

    #Connect to the new database

    \c my_db

    #Create the 'users' table

    CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    age INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
    ```
3. Create the `.env` file:

    In the root of your project directory, create a file named `.env` with the following content :
     ```sh
     # Database connection URL
     
     DATABASE_URL=postgres://your_username:your_password@localhost/my_db
     ```

     Replace your_username and your_password with your PostgreSQL credentials.

4. Run the Project:

    Before running the project, you need to prepare the database using `sqlx`:
    ```sh
    # Prepare the database for SQLx

    cargo sqlx prepare

    # Run the project

    cargo run
    ```
    The server should start at `localhost:3000`.

## Testing the Functionality

### Using Postman 

1. Open Postman and create a new request.

2. For the `POST` request to add a User :

    - Set the request type to `POST`
    - Enter the URL: `localhost:3000/users`
    - Set the request body to raw and select ``JSON`` format.
    - Enter the following `JSON` data :

    ```sh
     {
       "name": "John Doe",
       "email": "john.doe@example.com",
       "age": 30
    }   
    ```
    - Send the request 

        For the ``POST`` request, you should receive a response with the details you sent, confirming that they have been added to the database.

    We should have something like this :

![alt text](<Documentation/Screenshot 2024-07-03 034844.png>)

3. For the `GET` request to retrieve added Users :

    - Create a new request.
    - Set the request type to `GET`.
    - Enter the URL : `localhost:3000/users`
    - Send the request 

        For the ``GET`` request, you should receive a list of all the custom details stored in  the database.

    We should have something like this : 

![alt text](<Documentation/Screenshot 2024-07-03 035616.png>)