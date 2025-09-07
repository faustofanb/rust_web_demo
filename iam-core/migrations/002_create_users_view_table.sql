-- 创建用户读模型表
CREATE TABLE users_view (
    id BINARY(16) PRIMARY KEY,
    tenant_id BINARY(16) NOT NULL,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    created_at TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6),
    UNIQUE KEY uq_tenant_username (tenant_id, username),
    UNIQUE KEY uq_tenant_email (tenant_id, email)
);

-- 创建索引
CREATE INDEX idx_users_tenant_id ON users_view (tenant_id);
CREATE INDEX idx_users_status ON users_view (status);
CREATE INDEX idx_users_created_at ON users_view (created_at);
