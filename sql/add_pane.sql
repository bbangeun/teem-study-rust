INSERT INTO panes(page, structure)
VALUES ($1, $2)
RETURNING $table_fields;