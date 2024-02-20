# Record useful npm packages

- Feature Name: record-useful-npm-packages
- Start Date: 2024-01-24 21:32:54

## Summary

This RFC proposes the creation of a useful npm packages record tooling.

## Motivation

I hope to record npm packages that are useful, so that I can use them in the future. this tooling can record the package name, description, link, category, and the reason why I think it is useful.

## Reference-level explanation

Basic architecture:

![architecture](/sreenshot/record-package-arch.png)

### Events

I would use `@tauri-apps/api` `invoke` to send events to the backend.

```ts
invoke('addPackage', {
  name: string,
  description: string,
  link: string,
  category: string,
  reason: string,
})

invoke ('editPackage', {
  name: string,
  description: string | undefined,
  link: string | undefined,
  category: string | undefined,
  reason: string | undefined,
})

invoke('removePackage', {
  name: string,
})

invoke('queryPackages', {
  name: string | undefined,
  category: string | undefined,
  description: string | undefined,
  reason: string | undefined,
})
```

### Database

I would use `sqlite` to store the data.

```sql
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

CREATE TRIGGER update_packages_updated_at
AFTER UPDATE ON packages
FOR EACH ROW
BEGIN
  UPDATE packages SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- package name unique index
CREATE UNIQUE INDEX package_name_uindex ON packages (name);

-- package_categories table
CREATE TABLE package_categories (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name VARCHAR(64) NOT NULL,
  parent_id INTEGER DEFAULT 0 REFERENCES package_categories(id) ON DELETE CASCADE,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_package_categories_updated_at
AFTER UPDATE ON package_categories
FOR EACH ROW
BEGIN
  UPDATE package_categories SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- package_category_name and parent_id unique index
CREATE UNIQUE INDEX package_category_name_parent_id_uindex ON package_categories (name, parent_id);

-- package_category_relations table
CREATE TABLE package_category_relations (
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

```

## Drawbacks

N/A

## Prior art

N/A

## Unresolved questions

- Data

  - [ ] How to implement the backend?

## Future possibilities

- Online package use case (use sandbox ?)
  - [ ] How to implement the online package use case?
