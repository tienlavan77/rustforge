forgerust/
│
├── Cargo.toml
├── Cargo.lock
├── .env
├── .env.example
├── .gitignore
├── README.md
│
├── migrations/
│   └── (database migrations)
│
├── config/
│   └── (application configuration)
│
├── src/
│   │
│   ├── main.rs
│   ├── lib.rs
│   │
│   ├── bootstrap/
│   │   ├── mod.rs
│   │   └── startup.rs
│   │
│   ├── config/
│   │   ├── mod.rs
│   │   └── settings.rs
│   │
│   ├── server/
│   │   ├── mod.rs
│   │   ├── router.rs
│   │   └── middleware.rs
│   │
│   ├── api/
│   │   ├── mod.rs
│   │   └── health/
│   │       ├── mod.rs
│   │       ├── handler.rs
│   │       └── dto.rs
│   │
│   ├── modules/
│   │   ├── mod.rs
│   │   └── users/
│   │       ├── mod.rs
│   │       ├── domain/
│   │       ├── application/
│   │       ├── infrastructure/
│   │       └── presentation/
│   │
│   ├── infrastructure/
│   │   ├── mod.rs
│   │   ├── database/
│   │   ├── logging/
│   │   └── cache/
│   │
│   ├── shared/
│   │   ├── mod.rs
│   │   ├── error.rs
│   │   └── response.rs
│   │
│   └── workers/
│       └── mod.rs
│
└── tests/
    └── api/