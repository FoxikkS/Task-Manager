extern crate diesel;
mod schema;

use schema::tasks;
use diesel::prelude::*;
use diesel::dsl::max;
use diesel::pg::PgConnection;
use std::env;
use std::io;
use dotenv::dotenv;

#[derive(Insertable)]
#[table_name = "tasks"]
struct NewTask<'a> {
    id: i32,
    name_task: &'a str,
}

#[derive(Queryable)]
struct Task {
    id: i32,
    name_task: String,
}

fn main() {
    dotenv().ok();
    loop {
        let mut task = String::new();
        println!("Приветствую! Выберите действие из списка ниже..
1. Добавить задачу
2. Удалить задачу
3. Изменить задачу
4. Просмотреть задачи
5. Выход");

        match io::stdin().read_line(&mut task) {
            Ok(_) => {
                let task = task.trim();
                match task {
                    "1" => {
                        if let Err(e) = add_task() {
                            println!("Ошибка при добавлении задачи: {}", e);
                        }
                    }
                    "2" => {
                        if let Err(e) = del_task(){
                            println!("Ошибка при удалении задачи: {}", e);
                        }
                    }
                    "3" => {
                        if let Err(e) = izm_task(){
                            println!("Ошибка при изменении задачи: {}", e);
                        }
                    }
                    "4" => show_task(),
                    "5" => {
                        println!("Выход");
                        break;
                    }
                    _ => println!("Неверный выбор, попробуйте еще раз."),
                }
            }
            Err(e) => println!("ОШИБКА {}", e),
        }
    }
}

pub fn add_task() -> std::io::Result<()> {
    let mut task = String::new();
    println!("Введите задачу");
    io::stdin().read_line(&mut task)?;
    let task = task.trim();

    let mut connection = establish_connection();

    let max_id: Option<i32> = tasks::table
        .select(max(tasks::id))
        .first::<Option<i32>>(&mut connection)
        .expect("Ошибка при получении максимального ID");

    let new_id = match max_id {
        Some(id) => id + 1,
        None => 1,
    };

    let new_task = NewTask {
        id: new_id,
        name_task: task,
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(&mut connection)
        .expect("Не удалось добавить задачу! Повторите попытку.");
    println!("Задача {} добавлена!", task);
    Ok(())
}

pub fn del_task() -> std::io::Result<()> {
    let mut task_id = String::new();
    println!("Введите номер задачи, которую хотите удалить");
    io::stdin().read_line(&mut task_id)?;
    let task_id: i32 = task_id.trim().parse().expect("Введите корректный номер задачи!");

    let mut connection = establish_connection();

    match diesel::delete(tasks::table.find(task_id))
        .execute(&mut connection) {
        Ok(deleted_rows) if deleted_rows > 0 => {
            println!("Задача {} удалена!", task_id);
            diesel::update(tasks::table.filter(tasks::id.gt(task_id)))
                .set(tasks::id.eq(tasks::id - 1))
                .execute(&mut connection).expect("TODO: panic message");
            println!("Номера задач обновлены.");
        },
        Ok(_) => {
            println!("Задача с номером {} не найдена.", task_id);
        },
        Err(err) => {
            println!("Ошибка при удалении задачи: {:?}", err);
        }
    }
    Ok(())
}

pub fn izm_task() -> std::io::Result<()> {
    let mut task_id = String::new();
    println!("Введите номер задачи");
    io::stdin().read_line(&mut task_id)?;
    let task_id: i32 = task_id.trim().parse().expect("Введите корректный номер задачи!");

    let mut new_task = String::new();
    println!("Введите новую задачу");
    io::stdin().read_line(&mut new_task)?;
    let new_task = new_task.trim();

    let mut connection = establish_connection();

    diesel::update(tasks::table.find(task_id))
        .set(tasks::name_task.eq(new_task))
        .execute(&mut connection).expect("TODO: panic message");

    println!("Задача {} изменена на {}", task_id, new_task);

    Ok(())
}

pub fn show_task() {
    let mut connection = establish_connection();

    let tasks: Vec<Task> = tasks::table
        .order(tasks::id)
        .load(&mut connection)
        .expect("TODO: panic message");

    if tasks.is_empty() {
        println!("Список задач пуст.");
    } else {
        println!("Список задач:");
        for task in tasks {
            println!("{}: {}", task.id, task.name_task);
        }
    }
}

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}