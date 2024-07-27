-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
ALTER TABLE users
    ALTER COLUMN profile_image SET DEFAULT 'default_compressed.jpg';
