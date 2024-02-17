CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    display_name VARCHAR(50) NOT NULL,
    password_hashed TEXT NOT NULL,
    email VARCHAR(50) NOT NULL,
    created_at BIGINT NOT NULL
);