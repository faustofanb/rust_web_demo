
-- 插入测试数据
INSERT INTO users (username, email, password_hash) VALUES 
('admin', 'admin@example.com', '$argon2id$v=19$m=4096,t=3,p=1$c29tZXNhbHQ$iWh06vD8Fy27wf9npn6FXWiCX4K6pW6Ue1Bnzz07Z8A'),
('testuser', 'test@example.com', '$argon2id$v=19$m=4096,t=3,p=1$c29tZXNhbHQ$iWh06vD8Fy27wf9npn6FXWiCX4K6pW6Ue1Bnzz07Z8A');
