-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
ALTER TABLE users
    ALTER COLUMN profile_image SET DEFAULT 'default.jpg';

CREATE INDEX idx_users_username ON users (username);

CREATE INDEX idx_sessions_session_token ON sessions (session_token);
