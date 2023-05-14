-- Your SQL goes here
CREATE TABLE room (
  id bigserial NOT NULL PRIMARY KEY,
  item_id bigint DEFAULT NULL REFERENCES item(id),
  created_by bigint NOT NULL  REFERENCES users(id),
  created_at timestamp with time zone DEFAULT now() NOT NULL,
  deleted_at timestamp with time zone DEFAULT NULL
)