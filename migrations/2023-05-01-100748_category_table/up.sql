-- Your SQL goes here
CREATE TABLE category (
  id bigserial NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  avatar VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMP with time zone DEFAULT NULL
);

INSERT INTO category (name, avatar) VALUES
('Rings', 'category/ring.jpeg'),
('Earings', 'category/earing.jpeg'),
('Nosepins', 'category/noseping.jpeg'),
('Bracelets', 'category/bracelet.jpeg'),
('Necklaces', 'category/necklace.jpeg');