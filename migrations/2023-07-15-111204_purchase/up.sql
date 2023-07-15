-- Your SQL goes here
CREATE TABLE purchase (
  id bigserial NOT NULL PRIMARY KEY,
  buyer_id bigint NOT NULL  REFERENCES users(id),
  seller_id bigint NOT NULL  REFERENCES users(id),
  item_id bigint NOT NULL  REFERENCES item(id),
  created_at timestamp with time zone DEFAULT now() NOT NULL,
  unique(seller_id, item_id)
)