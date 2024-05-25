ALTER TABLE packages
ADD COLUMN synced INT DEFAULT 0;

ALTER TABLE package_categories
ADD COLUMN synced INT DEFAULT 0;

ALTER TABLE package_category_relations
ADD COLUMN synced INT DEFAULT 0;
