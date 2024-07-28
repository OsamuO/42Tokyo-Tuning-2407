-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
ALTER TABLE users
    ALTER COLUMN profile_image SET DEFAULT 'default.jpg';

CREATE INDEX idx_users_username ON users (username);

CREATE INDEX idx_sessions_session_token ON sessions (session_token);

-- edges テーブル
ALTER TABLE edges ADD INDEX idx_node_b_id (node_b_id);

-- locations テーブル
ALTER TABLE locations ADD INDEX idx_tow_truck_id (tow_truck_id);
ALTER TABLE locations ADD INDEX idx_node_id (node_id);
ALTER TABLE locations ADD INDEX idx_timestamp (timestamp);