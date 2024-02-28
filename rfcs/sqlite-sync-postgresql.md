# Local Sqlite sync to server Postgresql

- Feature Name: sqlite-sync-to-postgresql
- Start Date: 2024-02-28 10:08:39

## Summary

Sync local sqlite database to server postgresql database.

## Motivation

The data is stored in the local sqlite database, and I want to sync the data to the server postgresql database. So that I can access the data from anywhere.

## Reference-level explanation

I will display a button in the frontend to sync the data to the server. When the user clicks the button, the frontend will send a request to the backend to sync the data to the server. Importantly,the sync process is single direction, so don't need to consider the conflict currently.

### Events

When data syncing, some conditions need to be considered:

- The data is not synced to the server yet. I need insert the data to the server.
- The data is already synced to the server. I need to update the data to the server.
- The data is deleted in the local database. I need to delete the data in the server database.

### Database

Server `postgresql` database schema:

```sql
-- packages table
CREATE TABLE IF NOT EXISTS packages (
  id BIGSERIAL PRIMARY KEY,
  name VARCHAR(64) NOT NULL,
  description TEXT NOT NULL,
  link TEXT NOT NULL,
  reason TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- package name unique index
CREATE UNIQUE INDEX package_name_uindex ON packages (name);

CREATE OR REPLACE FUNCTION update_updated_at_func()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE TRIGGER update_updated_at_trigger
BEFORE UPDATE ON packages
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_func();

-- package_categories table
CREATE TABLE IF NOT EXISTS  package_categories (
  id BIGSERIAL PRIMARY KEY,
  name VARCHAR(64) NOT NULL,
  parent_id INTEGER DEFAULT 0,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- package_category_name and parent_id unique index
CREATE UNIQUE INDEX package_category_name_parent_id_uindex ON package_categories (name, parent_id);

CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE TRIGGER update_updated_at_trigger
BEFORE UPDATE ON package_categories
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

-- package_category_relations table
CREATE TABLE IF NOT EXISTS package_category_relations (
  id BIGSERIAL PRIMARY KEY,
  package_id INTEGER NOT NULL REFERENCES packages(id) ON DELETE CASCADE,
  category_id INTEGER NOT NULL REFERENCES package_categories(id) ON DELETE CASCADE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.update_at = CURRENT_TIMESTAMP;
  RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE TRIGGER update_updated_at_trigger
AFTER UPDATE ON package_category_relations
FOR EACH ROW EXECUTE PROCEDURE update_updated_at();

```

## Drawbacks

N/A

## Prior art

N/A

## Unresolved questions

N/A

## Future possibilities

- Two way sync
