-- Your SQL goes here
-- table to store media files of an item
CREATE TABLE item_image (
  id bigserial NOT NULL PRIMARY KEY,
  key VARCHAR NOT NULL,
  item_id bigint NOT NULL  REFERENCES item(id),
  user_id bigint NOT NULL  REFERENCES users(id),
  is_cover BOOLEAN NOT NULL DEFAULT FALSE,
  uploaded_to_cloud BOOLEAN NOT NULL DEFAULT FALSE,
  created_at timestamp with time zone DEFAULT now() NOT NULL,
  updated_at timestamp with time zone DEFAULT now() NOT NULL,
  deleted_at timestamp with time zone DEFAULT NULL,
  unique(key)
)