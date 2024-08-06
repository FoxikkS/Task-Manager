# Task-Manager
-----------------------------------------
How to use it?
It is necessary to download dotenv = "0.15.0" and diesel = "2.2.2" and postgres
After that, create a postgres database and insert a link to it in .env format "DATABASE_URL=postgres://user:password@localhost/namedatabase"
-----------------------------------------
Creating a diesel migration file
in console write "diesel setup"
after that, you will create a migration folder, you will see another folder in it, open it
there will be two files "down.sql" and "up.sql"
-----------------------------------------
in the "up.sql" file, add 

CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL
);
-----------------------------------------
in the "down.sql" file, add

DROP TABLE tasks;
-----------------------------------------
after these actions, enter the command "diesel migration run" in the console
