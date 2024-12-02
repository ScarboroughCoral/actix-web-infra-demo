-- Your SQL goes here
CREATE TABLE users (
                       id INTEGER PRIMARY KEY AUTOINCREMENT,        -- 用户主键
                       email VARCHAR(255) UNIQUE,                  -- 用户邮箱（可能为空）
                       username VARCHAR(255),                      -- 用户名（可从第三方同步）
                       avatar_url VARCHAR(512),                    -- 用户头像（GitHub 提供的头像 URL）
                       created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                       updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
);