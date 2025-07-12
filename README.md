# Customer Support Ticketing System

A comprehensive backend system built with **Rust**, **Axum**, and **PostgreSQL** for managing customer support operations with real-time collaboration features.

## Features

- **User Management**: Role-based access control (Admin, Agent, Customer)
- **Ticket Management**: Full CRUD operations with status tracking
- **Real-time Collaboration**: WebSocket support for live updates
- **Email Integration**: Automatic email notifications
- **Knowledge Base**: Searchable articles and FAQs
- **Comments System**: Internal notes and customer communication
- **Search & Filtering**: Advanced search capabilities
- **Authentication**: JWT-based authentication

## Tech Stack

- **Language**: Rust
- **Web Framework**: Axum
- **Database**: PostgreSQL with SeaORM
- **Authentication**: JWT
- **Real-time**: WebSockets
- **Email**: Lettre
- **Password Hashing**: bcrypt

## Prerequisites

- Rust (latest stable version)
- PostgreSQL 12+
- pgAdmin4 (for database management)

## Setup

### 1. Clone and Install Dependencies

```bash
git clone <repository-url>
cd major
cargo build
```

### 2. Database Setup

1. Create a PostgreSQL database:
```sql
CREATE DATABASE major_db;
```

2. Run the migration:
```bash
psql -d major_db -f migrations/20240101000000_create_initial_tables.sql
```

### 3. Environment Configuration

Create a `.env` file in the root directory:

```env
# Database
DATABASE_URL=postgres://username:password@localhost:5432/major_db

# JWT
JWT_SECRET=your_super_secret_jwt_key_here

# Email (Optional - for email notifications)
SMTP_SERVER=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your_email@gmail.com
SMTP_PASSWORD=your_app_password
FROM_EMAIL=your_email@gmail.com
```

### 4. Run the Application

```bash
cargo run
```

The server will start on `http://localhost:3000`

## API Documentation

### Authentication

#### Register User
```http
POST /auth/register
Content-Type: application/json

{
  "name": "John Doe",
  "email": "john@example.com",
  "password": "password123",
  "role": "customer"
}
```

#### Login
```http
POST /auth/login
Content-Type: application/json

{
  "email": "john@example.com",
  "password": "password123"
}
```

### Tickets

#### Create Ticket
```http
POST /tickets
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "subject": "Login Issue",
  "description": "I cannot log into my account",
  "priority": "High"
}
```

#### Get All Tickets
```http
GET /tickets?status=Open&priority=High&page=1&limit=10
Authorization: Bearer <jwt_token>
```

#### Update Ticket
```http
PUT /tickets/{ticket_id}
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "status": "In Progress",
  "assigned_to": "user-uuid-here"
}
```

### Comments

#### Add Comment
```http
POST /tickets/{ticket_id}/comments
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "content": "We are investigating this issue",
  "is_internal": true
}
```

#### Get Ticket Comments
```http
GET /tickets/{ticket_id}/comments
Authorization: Bearer <jwt_token>
```

### Knowledge Base

#### Create Article
```http
POST /knowledge-base
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "title": "How to Reset Password",
  "content": "Step by step guide...",
  "category": "Account",
  "tags": ["password", "reset", "account"]
}
```

#### Search Articles
```http
GET /knowledge-base?search=password&category=Account
Authorization: Bearer <jwt_token>
```

### WebSocket

Connect to WebSocket for real-time updates:

```javascript
const ws = new WebSocket('ws://localhost:3000/ws');

ws.onmessage = function(event) {
  const data = JSON.parse(event.data);
  console.log('Received:', data);
};
```

## Default Admin Account

The system comes with a default admin account:

- **Email**: admin@example.com
- **Password**: admin123

## Database Schema

### Users Table
- `id` (UUID, Primary Key)
- `name` (VARCHAR)
- `email` (VARCHAR, Unique)
- `password_hash` (VARCHAR)
- `role` (VARCHAR)
- `created_at` (TIMESTAMP)
- `updated_at` (TIMESTAMP)

### Tickets Table
- `id` (UUID, Primary Key)
- `subject` (VARCHAR)
- `description` (TEXT)
- `status` (VARCHAR)
- `priority` (VARCHAR)
- `assigned_to` (UUID, Foreign Key)
- `created_by` (UUID, Foreign Key)
- `created_at` (TIMESTAMP)
- `updated_at` (TIMESTAMP)

### Comments Table
- `id` (UUID, Primary Key)
- `ticket_id` (UUID, Foreign Key)
- `user_id` (UUID, Foreign Key)
- `content` (TEXT)
- `is_internal` (BOOLEAN)
- `created_at` (TIMESTAMP)

### Knowledge Base Table
- `id` (UUID, Primary Key)
- `title` (VARCHAR)
- `content` (TEXT)
- `category` (VARCHAR)
- `tags` (JSONB)
- `created_by` (UUID, Foreign Key)
- `created_at` (TIMESTAMP)
- `updated_at` (TIMESTAMP)

## Development

### Project Structure

```
src/
├── main.rs              # Application entry point
├── db.rs                # Database connection
├── models/              # SeaORM entities
│   ├── mod.rs
│   ├── user.rs
│   ├── ticket.rs
│   ├── comment.rs
│   └── knowledge_base.rs
├── auth/                # Authentication
│   ├── mod.rs
│   └── middleware.rs
├── handlers/            # Request handlers
│   ├── mod.rs
│   ├── auth.rs
│   ├── tickets.rs
│   ├── comments.rs
│   └── knowledge_base.rs
├── routes/              # Route definitions
│   └── mod.rs
├── ws/                  # WebSocket handling
│   └── mod.rs
└── email/               # Email service
    └── mod.rs
```

### Running Tests

```bash
cargo test
```

### Database Migrations

To create new migrations:

```bash
# Install sea-orm-cli
cargo install sea-orm-cli

# Generate migration
sea-orm-cli migrate generate <migration_name>

# Run migrations
sea-orm-cli migrate up
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

This project is licensed under the MIT License. 