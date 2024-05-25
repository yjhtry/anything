ALTER TABLE packages
DROP COLUMN synced;

ALTER TABLE package_categories
DROP COLUMN synced;

ALTER TABLE package_category_relations
DROP COLUMN synced;
