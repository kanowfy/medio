CREATE TABLE IF NOT EXISTS subscribers (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);