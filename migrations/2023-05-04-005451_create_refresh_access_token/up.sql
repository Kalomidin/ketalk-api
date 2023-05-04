-- Your SQL goes here
CREATE TABLE refresh_token (
  id bigserial NOT NULL PRIMARY KEY,
  user_id bigint NOT NULL  REFERENCES users(id),
  token VARCHAR NOT NULL,
  created_at timestamp with time zone DEFAULT now() NOT NULL,
  deleted_at timestamp with time zone DEFAULT NULL,
  unique(token)
)
