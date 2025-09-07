# 测试指南

本指南介绍 IAM Core 系统的测试策略、测试类型和如何运行测试。

## 🧪 测试类型

### 单元测试 (Unit Tests)
测试单个函数或模块的功能，不依赖外部资源。

**位置**: `src/tests/mod.rs`

**特点**:
- 快速执行
- 隔离测试
- 不依赖数据库
- 测试业务逻辑

### 集成测试 (Integration Tests)
测试多个组件之间的交互，包括数据库操作。

**位置**: `tests/integration_tests.rs`

**特点**:
- 需要数据库连接
- 测试完整流程
- 验证 API 接口
- 测试中间件

## 🚀 运行测试

### 运行所有测试
```bash
# 运行所有测试（包括单元测试和集成测试）
cargo test

# 只运行单元测试
cargo test --lib

# 只运行集成测试
cargo test --test integration_tests
```

### 运行特定测试
```bash
# 运行特定测试函数
cargo test test_user_registration

# 运行包含特定名称的测试
cargo test user

# 运行特定模块的测试
cargo test tests::user_aggregate_tests
```

### 测试选项
```bash
# 显示测试输出
cargo test -- --nocapture

# 并行运行测试
cargo test -- --test-threads=4

# 运行测试并生成覆盖率报告
cargo test -- --test-threads=1
```

## 📋 单元测试

### 用户聚合根测试

#### 测试用户注册
```rust
#[test]
fn test_user_registration() {
    let user_id = Uuid::new_v4();
    let tenant_id = Uuid::new_v4();
    let username = "testuser".to_string();
    let email = "test@example.com".to_string();
    let password_hash = "hashed_password".to_string();

    let event = User::register(user_id, tenant_id, username.clone(), email.clone(), password_hash.clone())
        .expect("User registration should succeed");

    match event {
        IdentityAccessEvent::UserRegistered(user_registered) => {
            assert_eq!(user_registered.user_id, user_id);
            assert_eq!(user_registered.tenant_id, tenant_id);
            assert_eq!(user_registered.username, username);
            assert_eq!(user_registered.email, email);
            assert_eq!(user_registered.password_hash, password_hash);
        }
        _ => panic!("Expected UserRegistered event"),
    }
}
```

#### 测试输入验证
```rust
#[test]
fn test_user_registration_validation() {
    let user_id = Uuid::new_v4();
    let tenant_id = Uuid::new_v4();

    // 测试空用户名
    let result = User::register(user_id, tenant_id, "".to_string(), "test@example.com".to_string(), "hash".to_string());
    assert!(result.is_err());

    // 测试无效邮箱
    let result = User::register(user_id, tenant_id, "username".to_string(), "invalid-email".to_string(), "hash".to_string());
    assert!(result.is_err());
}
```

#### 测试事件重建
```rust
#[test]
fn test_user_from_events() {
    let user_id = Uuid::new_v4();
    let tenant_id = Uuid::new_v4();
    let username = "testuser".to_string();
    let email = "test@example.com".to_string();
    let password_hash = "hashed_password".to_string();

    let event = User::register(user_id, tenant_id, username.clone(), email.clone(), password_hash.clone())
        .expect("User registration should succeed");

    let user = User::from_events(&[event]);

    assert_eq!(user.id(), user_id);
    assert_eq!(user.tenant_id(), tenant_id);
    assert_eq!(user.username(), username);
    assert_eq!(user.email(), email);
    assert_eq!(user.version(), 1);
}
```

## 🔗 集成测试

### 测试环境设置

#### 数据库配置
```rust
const TEST_DATABASE_URL: &str = "mysql://root:password@localhost:3306/iam_core_test";

async fn setup_test_app() -> axum::Router {
    // 加载测试配置
    let config = AppConfig {
        database: DatabaseConfig {
            url: TEST_DATABASE_URL.to_string(),
            max_connections: 5,
            min_connections: 1,
        },
        // ... 其他配置
    };

    // 连接数据库并运行迁移
    let pool = MySqlPool::connect(TEST_DATABASE_URL).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    // 创建应用
    create_router(app_state)
}
```

### API 测试

#### 健康检查测试
```rust
#[tokio::test]
async fn test_health_check() {
    let app = setup_test_app().await;

    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
```

#### 用户注册测试
```rust
#[tokio::test]
async fn test_user_registration() {
    let app = setup_test_app().await;

    let user_data = serde_json::json!({
        "username": "testuser",
        "email": "test@example.com",
        "password": "password123"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/users")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&user_data).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}
```

#### 输入验证测试
```rust
#[tokio::test]
async fn test_user_registration_validation() {
    let app = setup_test_app().await;

    let invalid_user_data = serde_json::json!({
        "username": "ab", // 太短
        "email": "test@example.com",
        "password": "password123"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/users")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&invalid_user_data).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
```

## 🛠️ 测试工具

### 测试数据库
```bash
# 创建测试数据库
mysql -u root -p -e "CREATE DATABASE iam_core_test;"

# 运行迁移
sqlx migrate run --database-url mysql://root:password@localhost:3306/iam_core_test
```

### 测试数据清理
```rust
// 在每个测试后清理数据
async fn cleanup_test_data() {
    let pool = MySqlPool::connect(TEST_DATABASE_URL).await.unwrap();
    
    // 清理测试数据
    sqlx::query("DELETE FROM events").execute(&pool).await.unwrap();
    sqlx::query("DELETE FROM users_view").execute(&pool).await.unwrap();
}
```

### Mock 和 Stub
```rust
// 使用 mock 对象进行测试
use mockall::*;

#[automock]
trait EventStore {
    async fn save_events(&self, aggregate_id: Uuid, events: &[IdentityAccessEvent], expected_version: u64) -> Result<(), AppError>;
    async fn load_events(&self, aggregate_id: Uuid) -> Result<Vec<StoredEvent>, AppError>;
}

#[tokio::test]
async fn test_user_service_with_mock() {
    let mut mock_store = MockEventStore::new();
    mock_store.expect_save_events()
        .times(1)
        .returning(|_, _, _| Ok(()));

    let user_service = UserService::new(Arc::new(mock_store));
    // 测试逻辑...
}
```

## 📊 测试覆盖率

### 生成覆盖率报告
```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html

# 查看报告
open tarpaulin-report.html
```

### 覆盖率目标
- **单元测试覆盖率**: > 80%
- **集成测试覆盖率**: > 60%
- **关键业务逻辑**: 100%

## 🔧 测试配置

### 测试环境变量
```env
# 测试数据库
TEST_DATABASE_URL=mysql://root:password@localhost:3306/iam_core_test

# 测试 JWT 密钥
TEST_JWT_SECRET=test-secret-key

# 测试环境
ENVIRONMENT=test
```

### 测试配置文件
```rust
// tests/test_config.rs
pub fn test_config() -> AppConfig {
    AppConfig {
        database: DatabaseConfig {
            url: std::env::var("TEST_DATABASE_URL").unwrap(),
            max_connections: 5,
            min_connections: 1,
        },
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 3001,
            cors_origins: vec!["*".to_string()],
        },
        jwt: JwtConfig {
            secret: "test-secret-key".to_string(),
            expiration_hours: 24,
        },
        environment: "test".to_string(),
    }
}
```

## 🚨 测试最佳实践

### 测试命名
- 使用描述性的测试名称
- 遵循 `test_<function>_<scenario>_<expected_result>` 格式
- 例如: `test_user_registration_with_valid_data_should_succeed`

### 测试结构
- **Arrange**: 准备测试数据
- **Act**: 执行被测试的操作
- **Assert**: 验证结果

### 测试隔离
- 每个测试应该独立运行
- 不依赖其他测试的结果
- 清理测试数据

### 错误测试
- 测试正常情况
- 测试边界条件
- 测试错误情况
- 测试异常情况

## 📝 测试检查清单

### 单元测试检查清单
- [ ] 所有公共函数都有测试
- [ ] 边界条件被测试
- [ ] 错误情况被测试
- [ ] 测试名称清晰
- [ ] 测试独立运行

### 集成测试检查清单
- [ ] API 端点被测试
- [ ] 数据库操作被测试
- [ ] 认证流程被测试
- [ ] 错误响应被测试
- [ ] 性能要求被验证

---

**需要帮助？** 查看 [API 文档](./api.md) 或提交 Issue。
