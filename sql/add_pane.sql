INSERT INTO panes(page_id, structure)
VALUES ($1, $2)
RETURNING $table_fields;