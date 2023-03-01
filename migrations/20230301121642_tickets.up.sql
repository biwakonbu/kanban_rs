-- Add up migration script here
CREATE TABLE IF NOT EXISTS tickets(
       id VARCHAR(128) NOT NULL PRIMARY KEY COMMENT 'ULID',
       user_id VARCHAR(128) NOT NULL COMMENT 'チケット所有ユーザID',
       status ENUM('todo', 'doing', 'done', 'pending') NOT NULL COMMENT 'チケットステータス',
       title VARCHAR(255) NOT NULL COMMENT 'チケット題名',
       description TEXT NOT NULL COMMENT 'チケット詳細',
       FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_ja_0900_as_cs;
