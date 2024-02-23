-- package_category_relations table
CREATE TABLE IF NOT EXISTS package_category_relations (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  package_id INTEGER NOT NULL REFERENCES packages(id) ON DELETE CASCADE,
  category_id INTEGER NOT NULL REFERENCES package_categories(id) ON DELETE CASCADE,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_package_category_relations_updated_at
AFTER UPDATE ON package_category_relations
FOR EACH ROW
BEGIN
  UPDATE package_category_relations SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
