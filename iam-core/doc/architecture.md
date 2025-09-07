# IAM Core 架构设计文档

## 1. 系统概述

本文档旨在设计一个基于 Rust 技术栈的、支持多租户的身份与访问管理（IAM）核心系统。系统将实现用户管理、角色与权限管理、菜单管理、组织与部门管理等功能。

### 1.1. 技术栈

- **Web 框架**: Axum (用于提供 API 接口)
- **数据库 ORM**: SQLx + SeaORM (用于读模型和事件存储)
- **消息队列**: Apache Kafka (用于领域事件的异步通知和解耦)
- **缓存**: Redis (用于缓存会话、权限等热点数据)
- **核心思想**: DDD, CQRS, Event Sourcing

## 2. 整体架构

系统将遵循分层架构，并严格遵守 DDD 和 CQRS 的设计原则。

![Architecture Diagram](https://user-images.githubusercontent.com/17799189/280312632-136330e2-7a87-41a1-885a-21165121e10c.png)


### 2.1. 各层职责

- **Interface (接口层)**: 使用 `Axum` 构建，负责处理 HTTP 请求，解析参数，调用应用层服务，并返回响应。同时处理认证、授权等中间件逻辑。
- **Application (应用层)**: 系统的入口，负责协调领域层对象完成业务。它包含处理 `Commands` 和 `Queries` 的服务。
    - **Command Side**: 接收 `Command`，加载聚合根，执行业务方法，并将产生的事件存入 Event Store。
    - **Query Side**: 接收 `Query`，直接查询由 `Projector` 生成的读模型（Read Models），返回 DTOs。
- **Domain (领域层)**: 系统的核心，包含所有业务逻辑。包括聚合根（Aggregates）、值对象（Value Objects）、领域事件（Domain Events）等。
- **Infrastructure (基础设施层)**: 提供技术实现。
    - **Event Store**: 事件存储的具体实现，使用 `SQLx` 将事件序列化后存入 PostgreSQL/MySQL。
    - **Projectors**: 订阅 `Event Store` 或 `Kafka` 中的事件流，将事件应用到读模型上，更新 `SeaORM` 管理的数据库表。
    - **Message Bus**: 将领域事件发布到 `Kafka`，供其他限界上下文或外部系统消费。
    - **Cache**: `Redis` 的实现，用于缓存权限数据、用户信息等。

## 3. 数据库表结构设计

我们将使用 PostgreSQL 数据库。

### 3.1. 事件存储表 (Write Side)

- **`events`**: 存储所有聚合产生的领域事件。

```sql
CREATE TABLE events (
    id BIGSERIAL PRIMARY KEY,
    aggregate_id UUID NOT NULL,
    aggregate_type VARCHAR(255) NOT NULL,
    event_type VARCHAR(255) NOT NULL,
    payload JSONB NOT NULL,
    version INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_aggregate_version UNIQUE (aggregate_id, version)
);
CREATE INDEX idx_events_aggregate_id ON events (aggregate_id);
CREATE INDEX idx_events_aggregate_type ON events (aggregate_type);
CREATE INDEX idx_events_event_type ON events (event_type);
```

### 3.2. 读模型表 (Read Side)

这些表由 Projector 维护，用于快速查询。

- **`tenants`**: 租户表
- **`users`**: 用户表
- **`roles`**: 角色表
- **`permissions`**: 权限表（或菜单表）
- **`organizations`**: 组织/部门表
- **关联表**: `user_roles`, `role_permissions`, `user_organizations`

```sql
-- 租户表
CREATE TABLE tenants (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active', -- active, inactive
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 用户表 (读模型)
CREATE TABLE users (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active', -- active, inactive, locked
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, username),
    UNIQUE (tenant_id, email)
);

-- 组织/部门表
CREATE TABLE organizations (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES organizations(id) ON DELETE SET NULL, -- 父级部门
    name VARCHAR(255) NOT NULL,
    code VARCHAR(100) NOT NULL, -- 组织编码
    level INT NOT NULL DEFAULT 0, -- 层级
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, code)
);

-- 角色表
CREATE TABLE roles (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    code VARCHAR(100) NOT NULL, -- 角色编码
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, code)
);

-- 权限/菜单表
CREATE TABLE permissions (
    id UUID PRIMARY KEY,
    parent_id UUID REFERENCES permissions(id) ON DELETE SET NULL,
    type VARCHAR(50) NOT NULL, -- 'menu', 'button', 'api'
    name VARCHAR(255) NOT NULL,
    code VARCHAR(100) NOT NULL UNIQUE, -- 全局唯一的权限标识
    route VARCHAR(255), -- 菜单路由
    icon VARCHAR(100), -- 菜单图标
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 用户-角色关联表
CREATE TABLE user_roles (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);

-- 角色-权限关联表
CREATE TABLE role_permissions (
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

-- 用户-组织关联表
CREATE TABLE user_organizations (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, organization_id)
);
```

## 4. 目录结构扩展

为了支持新功能，目录结构将进行如下扩展：

```
src
├── application
│   ├── services
│   │   ├── mod.rs
│   │   ├── user_service.rs
│   │   ├── role_service.rs      # 新增
│   │   └── organization_service.rs # 新增
│   └── dtos.rs
├── domain
│   ├── identity_access          # 用户、角色、权限
│   │   ├── aggregates
│   │   │   ├── user.rs
│   │   │   └── role.rs          # 新增
│   │   ├── commands.rs
│   │   ├── events.rs
│   │   └── ...
│   └── organization             # 组织架构
│       ├── aggregates
│       │   └── organization.rs  # 新增
│       ├── commands.rs          # 新增
│       └── events.rs            # 新增
└── infrastructure
    ├── persistence
    │   ├── event_store.rs
    │   ├── projectors           # 新增 Projectors 模块
    │   │   ├── mod.rs
    │   │   └── user_projector.rs
    │   └── read_models          # 新增 SeaORM 读模型
    │       ├── mod.rs
    │       ├── users.rs
    │       └── roles.rs
    └── messaging
        └── kafka_publisher.rs   # 新增
```
