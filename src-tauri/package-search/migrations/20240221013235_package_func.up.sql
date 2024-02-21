-- CREATE FUNCTION query_packages(
--   name: VARCHAR(64),
--   description: TEXT,
--   reason: TEXT,
--   categories: TEXT,
--   page: INTEGER DEFAULT 1,
--   page_size: INTEGER DEFAULT 10
-- )
-- RETURNS TABLE AS
-- DECLARE
--     _sql TEXT;
--     _where TEXT;


-- BEGIN
--     IF page < 1 THEN
--         page : = 1;
--     END IF;

--     IF page_size < 1 THEN
--         page_size : = 10;
--     END IF;
--     _where := format('%s%s%s',
--       CASE
--           WHEN name IS NOT NULL THEN format(' AND package.name like %%s%%', quote_literal(name))
--           ELSE ''
--       END,
--       CASE
--           WHEN description IS NOT NULL THEN format(' AND package.description like %%s%', quote_literal(description))
--           ELSE ''
--       END,
--       CASE
--           WHEN reason IS NOT NULL THEN format(' AND package.reason like %%s%', quote_literal(reason))
--           ELSE ''
--       END,
--     );


--     _sql : = format(
--       'SELECT package.*, pcl.category_id FROM packages AS package WHERE %s
--       LEFT JOIN package_category_relation AS pcl ON package.id = pcl.package_id %s LIMIT %L::integer OFFSET %L::integer',
--       CASE
--           WHEN LENGTH(TRIM(_where)) > 0 THEN _where
--           ELSE '1=1'
--       END,
--       CASE
--           WHEN categories IS NOT NULL THEN format('AND pcl.category_id IN (%s)', categories)
--           ELSE ''
--       END,
--       page_size,
--       (page - 1) * page_size
--     )

--     RETURN QUERY EXECUTE _sql;
-- END;
