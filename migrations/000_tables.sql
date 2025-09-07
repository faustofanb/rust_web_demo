CREATE TABLE users_view (
    id BINARY(16) PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP(6) NOT NULL,
    updated_at TIMESTAMP(6) NOT NULL
);

CREATE TABLE events (
    id BINARY(16) PRIMARY KEY,
    aggregate_id BINARY(16) NOT NULL,
    sequence BIGINT UNSIGNED NOT NULL,
    event_type VARCHAR(255) NOT NULL,
    payload JSON NOT NULL,
    created_at TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    UNIQUE KEY uq_aggregate_sequence (aggregate_id, sequence)
);