# Users REST API — Week 1 Assignment

A simple REST API built with **Rust** and the **Axum** framework that supports full CRUD (Create, Read, Update, Delete) operations on a list of users. Data is stored in memory — no database required.



## Prerequisites

Before running this project, make sure you have the following installed:


 Rust  [rustup.rs](https://rustup.rs) |
 Git  Version control [git-scm.com](https://git-scm.com) |

Verify your installations by running:

```bash
rustc --version
cargo --version
```

You should see version numbers printed for both. If not, restart your terminal after installing.



## Setup Instructions

### Step 1 — Clone the repository

```bash
git clone https://github.com/Krishna10724/users-api.git
cd users-api
```

### Step 2 — Build and run

```bash
cargo run
```

> The first time you run this, Cargo will download and compile all dependencies. This takes **2–3 minutes**. Subsequent runs are much faster.

### Step 3 — Confirm the server is running

You should see this message in your terminal:

```
Success!! Server is running at http://localhost:3000
   Available endpoints:
   GET    /users
   GET    /users/:id
   POST   /users
   PUT    /users/:id
   DELETE /users/:id
```

The server is now ready to accept requests on `http://localhost:3000`.



## API Endpoints

| Method | Endpoint | Description | Request Body |
|--------|----------|-------------|--------------|
| `GET` | `/users` | Get all users | None |
| `GET` | `/users/:id` | Get a user by ID | None |
| `POST` | `/users` | Create a new user | `{ "name": "...", "email": "..." }` |
| `PUT` | `/users/:id` | Update an existing user | `{ "name": "...", "email": "..." }` |
| `DELETE` | `/users/:id` | Delete a user | None |


## Testing the Endpoints

Open a **second terminal window** (keep the server running in the first one) and run the following commands.

### 1. Create a user

```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d "{\"name\": \"Alice\", \"email\": \"alice@example.com\"}"
```

**Expected response:**
```json
{
  "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "name": "Alice",
  "email": "alice@example.com"
}
```

> **Important:** Copy the `id` value from the response


### 2. Get all users

```bash
curl http://localhost:3000/users
```

**Expected response:**
```json
[
  {
    "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "name": "Alice",
    "email": "alice@example.com"
  }
]
```

### 3. Get a user by ID

Replace `YOUR-ID-HERE` with the `id` copied from Step 1.

```bash
curl http://localhost:3000/users/YOUR-ID-HERE
```

**Expected response:**
```json
{
  "id": "YOUR-ID-HERE",
  "name": "Alice",
  "email": "alice@example.com"
}
```



### 4. Update a user

Replace `YOUR-ID-HERE` with the same `id`.

```bash
curl -X PUT http://localhost:3000/users/YOUR-ID-HERE \
  -H "Content-Type: application/json" \
  -d "{\"name\": \"Alice Smith\", \"email\": \"alice.smith@example.com\"}"
```

**Expected response:**
```json
{
  "id": "YOUR-ID-HERE",
  "name": "Alice Smith",
  "email": "alice.smith@example.com"
}
```

### 5. Delete a user

Replace `YOUR-ID-HERE` with the same `id`.

```bash
curl -X DELETE http://localhost:3000/users/YOUR-ID-HERE
```

**Expected response:**
```
User 'YOUR-ID-HERE' deleted successfully
```

Run `GET /users` again to confirm the list is now empty.


## Project Structure

```
users-api/
├── Cargo.toml       # Project configuration and dependencies
├── Cargo.lock       # Locked dependency versions (auto-generated)
├── README.md        # This file
└── src/
    └── main.rs      # All API logic — routes, handlers, data structures
```


## Dependencies

| Library | Version | Purpose |
|---------|---------|---------|
| `axum` | 0.7 | Web framework — handles HTTP routing and requests |
| `tokio` | 1 | Async runtime — allows handling many requests simultaneously |
| `serde` | 1 | Serialization — converts Rust types to and from JSON |
| `serde_json` | 1 | JSON support for serde |
| `uuid` | 1 | Generates unique IDs for each user |

---

## Notes

- Data is stored **in memory only**. All users are lost when the server is restarted.
- The server runs on **port 3000** by default.
- User IDs are generated automatically using UUID v4 — you do not provide them when creating a user.
