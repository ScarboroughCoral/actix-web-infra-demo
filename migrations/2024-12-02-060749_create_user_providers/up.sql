-- Your SQL goes here
CREATE TABLE user_providers (
                                id INTEGER PRIMARY KEY AUTOINCREMENT,        -- 主键
                                user_id INTEGER NOT NULL,                     -- 关联到 `users` 表的用户 ID
                                provider VARCHAR(50) NOT NULL,               -- 第三方登录平台（例如 'github'）
                                provider_user_id VARCHAR(255) NOT NULL,      -- 第三方平台的用户唯一 ID
                                access_token TEXT,                           -- 第三方登录返回的 access_token（可选，视需求存储）
                                refresh_token TEXT,                          -- 第三方登录返回的 refresh_token（可选，视需求存储）
                                expires_at TIMESTAMP NULL,                   -- access_token 过期时间（可选，视需求存储）
                                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 更新时间
                                UNIQUE (provider, provider_user_id),         -- 确保每个第三方用户 ID 在特定平台唯一
                                FOREIGN KEY (user_id) REFERENCES users(id)   -- 外键约束
);