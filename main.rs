

        use std::env;
        use handler::{
            create_todo_handler, delete_todo_handler, edit_todo_handler, get_todo_handler,
            health_checker_handler, todos_list_handler,
        };
         
        #[macro_use]
        extern crate rocket;
         
        mod handler;
        mod model;
        mod response;
         
        #[launch]
        fn rocket() -> _ {
            let app_data = model::AppState::init();
         
            // Retrieve LOCALHOST_ADDRESS as a String
            let rustrocket_local_ip = match env::var("LOCALHOST_ADDRESS") {
                Ok(ip) => ip,
                Err(_) => "127.0.0.1".to_string(),
            };
         
            // Retrieve ROCKET_PORT, parse it to u16, and provide a default if needed
            let rustrocket_local_port = match env::var("ROCKET_PORT") {
                Ok(port_str) => match port_str.parse::<u16>() {
                    Ok(port) => port,
                    Err(_) => 8000, // Default if parsing fails or variable is missing
                },
                Err(_) => 8000, // Default if environment variable is missing
            };
         
         
         
            println!(" d-_-P The server is ready to accept requests at {}:{}/api/healthchecker", rustrocket_local_ip, rustrocket_local_port);
         
         
            rocket::build().manage(app_data).mount(
                "/api",
                routes![
                    health_checker_handler,
                    todos_list_handler,
                    create_todo_handler,
                    get_todo_handler,
                    edit_todo_handler,
                    delete_todo_handler
                ],
            )
        }

