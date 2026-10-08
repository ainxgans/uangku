# Uangku — Catatan Keuangan

## Overview

Aplikasi catatan keuangan multi-user dengan auth email OTP. Stack: Loco (Rust) backend, SvelteKit frontend, PostgreSQL, Docker. Deploy target: cloud VPS.

## Architecture

Monorepo dengan separation backend API dan frontend SSR.

```
uangku/
├── backend/                # Loco Rust (REST API, SeaORM, Axum)
│   ├── src/
│   │   ├── controllers/    # HTTP handlers
│   │   ├── models/         # SeaORM entities
│   │   ├── mailers/        # OTP email
│   │   └── workers/        # Background jobs (export)
│   └── migration/
├── frontend/               # SvelteKit (SSR)
│   └── src/
│       ├── routes/
│       ├── lib/
│       └── components/
├── docker-compose.yml
├── Dockerfile.backend
└── Dockerfile.frontend
```

- Backend: Loco framework (SeaORM + Axum), port 3000
- Frontend: SvelteKit SSR (adapter-node), port 3001
- DB: PostgreSQL 16

## Data Model

### users
| Column | Type | Notes |
|---|---|---|
| id | UUID PK | |
| email | VARCHAR | UNIQUE |
| name | VARCHAR | |
| created_at | TIMESTAMP | |
| updated_at | TIMESTAMP | |

### otp_codes
| Column | Type | Notes |
|---|---|---|
| id | UUID PK | |
| user_id | UUID FK | → users |
| code | VARCHAR | 6 digit |
| expires_at | TIMESTAMP | |
| used | BOOLEAN | default false |

### categories
| Column | Type | Notes |
|---|---|---|
| id | UUID PK | |
| user_id | UUID FK | → users |
| name | VARCHAR | |
| type | ENUM | income / expense |
| icon | VARCHAR | optional |
| created_at | TIMESTAMP | |

### transactions
| Column | Type | Notes |
|---|---|---|
| id | UUID PK | |
| user_id | UUID FK | → users |
| category_id | UUID FK | → categories |
| type | ENUM | income / expense |
| amount | BIGINT | Rupiah, tanpa desimal |
| description | VARCHAR | optional |
| date | DATE | |
| created_at | TIMESTAMP | |
| updated_at | TIMESTAMP | |

### budgets
| Column | Type | Notes |
|---|---|---|
| id | UUID PK | |
| user_id | UUID FK | → users |
| category_id | UUID FK | → categories |
| month | DATE | first day of month |
| limit_amount | BIGINT | Rupiah |
| created_at | TIMESTAMP | |
| updated_at | TIMESTAMP | |

UNIQUE(user_id, category_id, month) on budgets.

## API Endpoints

### Auth
- `POST /api/auth/request-otp` { email } → 200
- `POST /api/auth/verify-otp` { email, code } → JWT httpOnly cookie
- `POST /api/auth/logout` → clear cookie
- `GET /api/auth/me` → user profile

### Categories CRUD
- `GET /api/categories`
- `POST /api/categories` { name, type, icon? }
- `PUT /api/categories/:id` { name, icon? }
- `DELETE /api/categories/:id`

### Transactions CRUD
- `GET /api/transactions` ?month=&category_id=&type=
- `POST /api/transactions` { category_id, type, amount, description?, date }
- `PUT /api/transactions/:id`
- `DELETE /api/transactions/:id`

### Budgets CRUD
- `GET /api/budgets` ?month=
- `POST /api/budgets` { category_id, month, limit_amount }
- `PUT /api/budgets/:id`
- `DELETE /api/budgets/:id`

### Dashboard
- `GET /api/dashboard/summary` ?month= → total income, expense, balance
- `GET /api/dashboard/by-category` ?month= → per kategori + budget usage %

### Export
- `GET /api/export/csv` ?from=&to=
- `GET /api/export/pdf` ?from=&to=

## Frontend Pages

| Path | Deskripsi |
|---|---|
| / | Landing → redirect /dashboard if logged in |
| /login | Email → OTP flow |
| /dashboard | Summary cards, Chart.js bar chart, budget warnings |
| /transactions | List + filters, add/edit/delete |
| /transactions/new | Add transaction form |
| /budgets | Budget per kategori, progress bars |
| /categories | Manage categories CRUD |
| /export | Date range picker, CSV/PDF download |
| /settings | Edit profile name |

## Docker Setup

3 services: postgres:16-alpine, backend (multi-stage Rust build), frontend (Node SSR).
`.env` for secrets. Health checks + restart policies for production.

## Decisions

- BIGINT for amount (Rupiah, no decimals)
- JWT in httpOnly cookie
- Email OTP auth (no passwords)
- Chart.js for dashboard charts
- Loco workers for PDF export
- Default categories seeded on registration
- UUID v4 PKs, UTC timestamps
- JWT expiry 7 days, OTP expiry 5 minutes
