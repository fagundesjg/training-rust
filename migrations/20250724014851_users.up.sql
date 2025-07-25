CREATE TABLE users (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  email TEXT NOT NULL UNIQUE,
  birth_date TEXT NOT NULL,
  gender TEXT NOT NULL CHECK (gender IN ('male', 'female', 'other')),
  password TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT (DATETIME('now')),
  updated_at TEXT NULL
);

CREATE TRIGGER trigger_update_users_updated_at
AFTER UPDATE ON users
FOR EACH ROW
BEGIN
  UPDATE users
  SET updated_at = DATETIME('now')
  WHERE id = OLD.id;
END;
