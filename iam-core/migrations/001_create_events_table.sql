-- 创建事件存储表
CREATE TABLE events (
    id BINARY(16) PRIMARY KEY,
    aggregate_id BINARY(16) NOT NULL,
    sequence BIGINT UNSIGNED NOT NULL,
    event_type VARCHAR(255) NOT NULL,
    payload JSON NOT NULL,
    created_at TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    UNIQUE KEY uq_aggregate_sequence (aggregate_id, sequence)
);

-- 创建索引
CREATE INDEX idx_events_aggregate_id ON events (aggregate_id);
CREATE INDEX idx_events_aggregate_type ON events (event_type);
CREATE INDEX idx_events_created_at ON events (created_at);
