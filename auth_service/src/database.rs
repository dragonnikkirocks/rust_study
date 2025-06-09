
    pub mod connection{
        pub fn connection_status() -> Bool {
            // Simulating a database connection status check
            true
        }

        pub fn connect(new_user -> User) -> Result<(), String>{
            // Simulating a database connection and user creation
            if connection_status() {
                println!("Connected user to database:");
                println!("User ID: {}", new_user.id);
                Ok(())
           }else {
                Err("Failed to connect to the database".to_string())
            
            }
        }
    }