-- Your SQL goes here
CREATE TABLE item (
  id bigserial NOT NULL PRIMARY KEY,
  description VARCHAR NOT NULL,
  details VARCHAR NOT NULL,
  price bigint NOT NULL,
  negotiable boolean NOT NULL DEFAULT false,  
  owner_id bigint NOT NULL  REFERENCES users(id),
  favorite_count bigint NOT NULL DEFAULT 0,
  message_count bigint NOT NULL DEFAULT 0,
  seen_count bigint NOT NULL DEFAULT 0,
  created_at timestamp with time zone DEFAULT now() NOT NULL,
  updated_at timestamp with time zone DEFAULT now() NOT NULL,
  deleted_at timestamp with time zone DEFAULT NULL
)