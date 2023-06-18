# ketalk-api

Welcome to the ketalk-api repository! This repository contains the backend API for the Ketalk application. Ketalk is a messaging platform that allows users to communicate in real-time through text messages.

## Technologies Used

- Rust
- Actix Web
- PostgreSQL
- Diesel

## Features

- User authentication: The API provides user authentication functionality, allowing users to register, log in, and log out.
- Real-time messaging: Utilizing WebSocket and Actix channels, the API enables real-time communication between users, supporting instant messaging with features such as sending and receiving messages, online status updates, and typing indicators.
- Message history: The API stores message history in a PostgreSQL database, allowing users to access their past conversations.
- User profile management: Users can view and update their profile information, including their name, profile picture, and other details.

## Getting Started

To get started with the ketalk-api:

1. Clone the repository:

```
git clone https://github.com/Kalomidin/ketalk-api.git
```

2. Install Rust and Cargo:
Follow the instructions at rustup.rs to install Rust and Cargo, the Rust package manager.
3. Set up the PostgreSQL database:
Create a PostgreSQL database and update the database configuration in the Rocket.toml file with the appropriate connection URL.
4. Run database migrations:
```
diesel migration run
```
5. Start the server:
```
cargo run
```
The API server will start running on http://localhost:8000.

## API Documentation
The API endpoints and their usage are documented in the API Documentation file. Please refer to it for more details on the available routes and their functionalities.

## Contributing
Contributions are welcome! If you find any issues or want to suggest improvements, please feel free to submit an issue or a pull request.



