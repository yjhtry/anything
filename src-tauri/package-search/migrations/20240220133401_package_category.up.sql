-- package_categories table
CREATE TABLE package_categories (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name VARCHAR(64) NOT NULL,
  parent_id INTEGER DEFAULT 0,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- package_category_name and parent_id unique index
CREATE UNIQUE INDEX package_category_name_parent_id_uindex ON package_categories (name, parent_id);

CREATE TRIGGER update_package_categories_updated_at
AFTER UPDATE ON package_categories
FOR EACH ROW
BEGIN
  UPDATE package_categories SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
