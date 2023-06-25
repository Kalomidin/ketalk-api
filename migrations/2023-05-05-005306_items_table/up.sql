-- Your SQL goes here
CREATE TABLE item (
  id bigserial NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  price bigint NOT NULL,
  negotiable boolean NOT NULL DEFAULT false,  
  owner_id bigint NOT NULL  REFERENCES users(id),
  item_status VARCHAR NOT NULL DEFAULT 'Active',
  is_hideen boolean NOT NULL DEFAULT false,
  favorite_count int NOT NULL DEFAULT 0,
  message_count int NOT NULL DEFAULT 0,
  seen_count int NOT NULL DEFAULT 0,
  size float NOT NULL,
  weight float NOT NULL,
  karat_id bigint NOT NULL REFERENCES karat(id),
  category_id bigint NOT NULL REFERENCES category(id),
  geofence_id bigint NOT NULL REFERENCES geofence(id),
  created_at timestamp with time zone DEFAULT now() NOT NULL,
  updated_at timestamp with time zone DEFAULT now() NOT NULL,
  deleted_at timestamp with time zone DEFAULT NULL
)