UPDATE panes
SET page_id = $1, structure = $2
WHERE page_id = $1
RETURNING $table_fields;