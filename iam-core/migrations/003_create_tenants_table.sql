-- 创建租户表
CREATE TABLE IF NOT EXISTS tenants (
    id BINARY(16) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    code VARCHAR(100) NOT NULL UNIQUE,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    created_at TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6)
);

-- 创建索引
CREATE INDEX idx_tenants_code ON tenants (code);
CREATE INDEX idx_tenants_status ON tenants (status);
