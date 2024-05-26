CREATE TABLE IF NOT EXISTS del_records (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  -- 1 is package, 2 is category, 3 is package and category relation
  type INTEGER NOT NULL,
  del_id INTEGER
);
