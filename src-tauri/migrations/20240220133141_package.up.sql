-- packages table
CREATE TABLE packages (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name VARCHAR(64) NOT NULL,
  description TEXT NOT NULL,
  link TEXT NOT NULL,
  reason TEXT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- package name unique index
CREATE UNIQUE INDEX package_name_uindex ON packages (name);

CREATE TRIGGER update_packages_updated_at
AFTER UPDATE ON packages
FOR EACH ROW
BEGIN
  UPDATE packages SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
