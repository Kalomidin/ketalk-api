-- Your SQL goes here
CREATE TABLE users (
  id bigserial NOT NULL PRIMARY KEY,
  user_name VARCHAR NOT NULL,
  phone_number VARCHAR NOT NULL,
  -- created_at timestamp with time zone DEFAULT now() NOT NULL,
  -- updated_at timestamp with time zone DEFAULT now() NOT NULL,
  unique(phone_number)
)
