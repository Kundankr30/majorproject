# Customer Support Ticketing System

A comprehensive, enterprise-grade customer support ticketing system built with **Rust**, **Axum**, and **PostgreSQL**. Designed to handle multi-channel customer inquiries with real-time collaboration, automated email processing, and advanced analytics.

## 🎯 Problem Statement

Build a scalable backend system that efficiently manages customer support operations across multiple communication channels. The system provides tools for support agents to respond effectively while maintaining complete conversation history and enabling real-time collaboration.

## 🚀 Core Features

### 🔐 User Management & Role-Based Access
- **Multi-role Support**: Admin, Agent, Customer roles with granular permissions
- **Agent Profiles**: Individual login credentials and performance tracking
- **Permission Management**: Role-based access control for system features

### 🎫 Comprehensive Ticket Management
- **Full CRUD Operations**: Create, read, update, delete support tickets
- **Status Workflow**: Open → In Progress → Pending → Resolved → Closed
- **Priority Levels**: Low, Medium, High, Critical with SLA tracking
- **Agent Assignment**: Intelligent ticket routing and manual assignment
- **SLA Management**: Response time tracking and escalation rules

### 📧 Email Integration & Automation
- **Email-to-Ticket Conversion**: Automatic ticket creation from incoming emails
- **Direct Email Replies**: Agents can respond directly from ticket interface
- **Email Templates**: Predefined response templates for common scenarios
- **Auto-Notifications**: Status updates and follow-up emails to customers

### 🌐 Multi-Channel Communication
- **Unified Interface**: Handle email, live chat, and social media in one platform
- **Channel Integration**: Support for multiple communication platforms
- **Conversation Threading**: Maintain context across different channels
- **Channel-Specific Features**: Optimized workflows for each channel type

### ⚡ Real-Time Collaboration
- **Live Updates**: Real-time ticket status changes and notifications
- **Typing Indicators**: Show when agents are composing responses
- **Conflict Resolution**: Handle concurrent edits and prevent conflicts
- **Agent Presence**: Show which agents are currently active
- **WebSocket Integration**: Instant communication between agents

### 💬 Customer Communication Portal
- **Ticket Status Tracking**: Customers can check ticket progress
- **Follow-up Messages**: Add additional information to existing tickets
- **Conversation History**: Complete audit trail of all interactions
- **Customer Self-Service**: Knowledge base access and FAQ resolution

### 📝 Internal Collaboration Tools
- **Internal Notes**: Private comments visible only to agents
- **Team Collaboration**: Share insights and coordinate responses
- **Knowledge Sharing**: Document solutions and best practices
- **Agent Comments**: Structured communication between team members

### 📚 Knowledge Base Management
- **Searchable Articles**: Comprehensive FAQ and solution database
- **Category Organization**: Structured content management
- **Tag System**: Advanced content tagging and filtering
- **Auto-Suggestions**: Relevant articles based on ticket content
- **Analytics**: Track article usage and effectiveness

### 📊 Advanced Reporting & Analytics
- **Performance Metrics**: Response times, resolution rates, customer satisfaction
- **Agent Analytics**: Individual and team performance tracking
- **Trend Analysis**: Historical data and pattern recognition
- **Custom Dashboards**: Configurable reporting interfaces
- **Export Capabilities**: Data export in multiple formats

### 🔍 Powerful Search & Filtering
- **Full-Text Search**: Search across tickets, comments, and knowledge base
- **Advanced Filters**: Status, priority, agent, date range, custom criteria
- **Saved Searches**: Store frequently used search queries
- **Search Analytics**: Track search patterns and popular queries

## 🛠 Tech Stack

- **Backend**: Rust + Axum (High-performance async web framework)
- **Database**: PostgreSQL + SeaORM (Type-safe ORM)
- **Authentication**: JWT + bcrypt (Secure password hashing)
- **Real-time**: WebSockets (Live collaboration)
- **Email**: Lettre (SMTP integration)
- **Validation**: Serde (Data serialization/deserialization)
- **Async Runtime**: Tokio (Non-blocking I/O)





Server starts at `http://localhost:3000`

## 📚 API Reference


```
http://localhost:3000
```

### Authentication

#### Register User
```http
POST /auth/register
Content-Type: application/json

{
  "name": "Kundan Kumar",
  "email": "kundanixr@gmail.com",
  "password": "password123",
  "role": "agent"
}
```

#### Login
```http
POST /auth/login
Content-Type: application/json

{
  "email": "kundanixr@gmail.com",
  "password": "password123"
}
```

**Response:**
```json
{
  "token": "jwt_token_here",
  "user": {
    "id": "uuid",
    "name": "Kundan Kumar",
    "email": "Kundanixr@gmail.com",
    "role": "agent"
  }
}
```

### Tickets

#### Create Ticket
```http
POST /tickets
Authorization: Bearer <token>
Content-Type: application/json

{
  "subject": "Login Issue",
  "description": "Cannot access account",
  "priority": "High",
  "channel": "email",
  "customer_email": "customer@example.com"
}
```

#### List Tickets (with advanced filtering)
```http
GET /tickets?status=Open&priority=High&assigned_to=agent-uuid&page=1&limit=10&search=login
Authorization: Bearer <token>
```

#### Get Ticket Details
```http
GET /tickets/{ticket_id}
Authorization: Bearer <token>
```

#### Update Ticket Status
```http
PUT /tickets/{ticket_id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "status": "In Progress",
  "assigned_to": "agent-uuid",
  "priority": "Medium"
}
```

#### Delete Ticket
```http
DELETE /tickets/{ticket_id}
Authorization: Bearer <token>
```

### Comments & Internal Notes

#### Add Public Comment
```http
POST /tickets/{ticket_id}/comments
Authorization: Bearer <token>
Content-Type: application/json

{
  "content": "We are investigating this issue",
  "is_internal": false
}
```

#### Add Internal Note
```http
POST /tickets/{ticket_id}/comments
Authorization: Bearer <token>
Content-Type: application/json

{
  "content": "Internal investigation notes - checking server logs",
  "is_internal": true
}
```

#### Get Ticket Comments
```http
GET /tickets/{ticket_id}/comments
Authorization: Bearer <token>
```

### Knowledge Base

#### Create Article
```http
POST /knowledge-base
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "How to Reset Password",
  "content": "Step by step guide for password reset...",
  "category": "Account Management",
  "tags": ["password", "reset", "account"],
  "is_public": true
}
```

#### Search Knowledge Base
```http
GET /knowledge-base?search=password&category=Account&page=1&limit=10
Authorization: Bearer <token>
```

#### Get Article
```http
GET /knowledge-base/{article_id}
Authorization: Bearer <token>
```

#### Update Article
```http
PUT /knowledge-base/{article_id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "Updated Password Reset Guide",
  "content": "Updated step-by-step instructions...",
  "tags": ["password", "reset", "account", "security"]
}
```

### Real-Time WebSocket

Connect for real-time collaboration:

```javascript
const ws = new WebSocket('ws://localhost:3000/ws');

ws.onmessage = function(event) {
  const data = JSON.parse(event.data);
  console.log('Received:', data);
};

// Send typing indicator
ws.send(JSON.stringify({
  type: "TypingIndicator",
  ticket_id: "ticket-uuid",
  user_id: "user-uuid",
  is_typing: true
}));
```

**Message Types:**
- `TicketUpdate`: Real-time ticket status changes
- `TypingIndicator`: User typing notifications
- `NewComment`: New comment notifications
- `AgentPresence`: Agent online/offline status
- `TicketAssignment`: Real-time assignment updates

## 🔧 System Health

```http
GET /health
```

Returns: `"OK"`

## 📊 Data Models & Business Logic

### User Roles & Permissions
- **Admin**: Full system access, user management, analytics
- **Agent**: Ticket management, customer communication, knowledge base access
- **Customer**: Create tickets, view own tickets, access public knowledge base

### Ticket Status Workflow
- `Open` → `In Progress` → `Pending` → `Resolved` → `Closed`
- Automatic escalation based on SLA violations
- Status change notifications to customers

### Priority Levels & SLA
- `Low`: 48-hour response time
- `Medium`: 24-hour response time  
- `High`: 4-hour response time
- `Critical`: 1-hour response time

### Communication Channels
- **Email**: Direct email integration with ticket creation
- **Live Chat**: Real-time chat support
- **Social Media**: Integration with social platforms
- **Phone**: Manual ticket creation from phone calls

## 🚀 Development

### Project Structure
```
src/
├── main.rs              # Application entry point
├── db.rs                # Database connection & configuration
├── auth/                # Authentication & authorization
│   ├── mod.rs           # JWT token management
│   └── middleware.rs    # Auth middleware
├── handlers/            # API endpoint handlers
│   ├── mod.rs           # Handler exports
│   ├── auth.rs          # Authentication endpoints
│   ├── tickets.rs       # Ticket management
│   ├── comments.rs      # Comments & internal notes
│   └── knowledge_base.rs # Knowledge base management
├── models/              # Database models & entities
│   ├── mod.rs           # Model exports
│   ├── user.rs          # User entity
│   ├── ticket.rs        # Ticket entity
│   ├── comment.rs       # Comment entity
│   └── knowledge_base.rs # Knowledge base entity
├── routes/              # Route definitions
│   └── mod.rs           # Route configuration
├── ws/                  # WebSocket handlers
│   └── mod.rs           # Real-time communication
└── email/               # Email service
    └── mod.rs           # SMTP integration
''

## 📈 Performance & Scalability

- **Async Architecture**: Non-blocking I/O for high concurrency
- **Database Optimization**: Efficient queries with proper indexing
- **Connection Pooling**: Optimized database connections
- **Caching Strategy**: Redis integration for frequently accessed data
- **Load Balancing**: Horizontal scaling support

## 🔒 Security Features

- **JWT Authentication**: Secure token-based authentication
- **Password Hashing**: bcrypt for secure password storage
- **Input Validation**: Comprehensive request validation
- **SQL Injection Prevention**: Parameterized queries
- **Rate Limiting**: API rate limiting to prevent abuse
- **CORS Configuration**: Cross-origin resource sharing set