-- Your SQL goes here
CREATE TABLE user_favorite (
  id bigserial NOT NULL PRIMARY KEY,
  user_id bigint NOT NULL  REFERENCES users(id),
  item_id bigint NOT NULL  REFERENCES item(id),
  is_favorite BOOLEAN NOT NULL DEFAULT TRUE,
  created_at timestamp with time zone DEFAULT now() NOT NULL,
  updated_at timestamp with time zone DEFAULT now() NOT NULL
)