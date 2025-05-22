
# Red Dot - Incident Reporting App Backend

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![Actix-web](https://img.shields.io/badge/Actix--web-00A9FF?style=flat)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-336791?style=flat&logo=postgresql&logoColor=white)

---

## Overview

**Red Dot** is a backend API for a location-based incident reporting app built in Rust. Users can anonymously submit and track minor incidents (e.g., thefts, vandalism) with precise geolocation data. The backend uses Actix-web for HTTP server functionality, Postgres with PostGIS for spatial queries, and SQLx for async database interactions.

---

## Features

- Submit incidents with title, description, category, and GPS coordinates
- Retrieve nearby incidents based on location and radius
- Supports anonymous reporting with moderation-ready design
- Geospatial queries powered by PostGIS for accurate location filtering
- Rust async backend for performance and safety

---

## Tech Stack

- Rust
- Actix-web (web framework)
- SQLx (async DB client)
- PostgreSQL + PostGIS (database with spatial support)
- Tokio (async runtime)
- Chrono (date/time handling)

---

## Getting Started

### Prerequisites

- Rust toolchain (1.70+ recommended)
- PostgreSQL with PostGIS extension installed and enabled
- `cargo` CLI installed

### Setup Database

1. Create your Postgres database:

   ```sql
   CREATE DATABASE red_dot;
   \c red_dot
   CREATE EXTENSION postgis;
   ```

2. Run migrations to create necessary tables:

   *(Add your migration instructions here)*

---

### Run Locally

1. Clone the repo

   ```bash
   git clone https://github.com/AncutaIoan/red-dot.git
   cd red-dot
   ```

2. Configure environment variables (example `.env`):

   ```env
   DATABASE_URL=postgres://user:password@localhost/red_dot
   ```

3. Build and run the server

   ```bash
   cargo run
   ```

4. The API will be available at `http://localhost:8080`

---

## API Endpoints

| Method | Path              | Description                  |
|--------|-------------------|------------------------------|
| POST   | `/incident/add`   | Add a new incident report    |
| GET    | `/incident/nearby`| Retrieve incidents near a location |

Example request to add an incident:

```http
POST /incident/add
Content-Type: application/json

{
  "title": "Suspicious activity near park",
  "description": "Saw someone attempting to break into a bike rack.",
  "category": "theft",
  "latitude": 47.1585,
  "longitude": 27.6014
}
```

---

## Contributing

Contributions are welcome! Please open issues or submit pull requests for bug fixes and enhancements.

---

## License

MIT License © 2025 Your Name

---

## Contact

Created by Your Name — [your.email@example.com](mailto:your.email@example.com)
