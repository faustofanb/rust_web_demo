-- 组织/部门表
CREATE TABLE IF NOT EXISTS organizations (
    id CHAR(36) NOT NULL PRIMARY KEY,
    tenant_id CHAR(36) NOT NULL,
    parent_id CHAR(36),
    name VARCHAR(255) NOT NULL,
    code VARCHAR(100) NOT NULL,
    level INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE (tenant_id, code)
);

-- 用户-组织关联表
CREATE TABLE user_organizations (
    user_id CHAR(36) NOT NULL,
    organization_id CHAR(36) NOT NULL,
    PRIMARY KEY (user_id, organization_id),
);
