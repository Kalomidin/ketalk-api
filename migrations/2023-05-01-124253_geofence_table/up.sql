-- Your SQL goes here
CREATE TABLE geofence (
  id bigserial NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  geofence_type VARCHAR NOT NULL,
  parent_region_id  bigint NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMP with time zone DEFAULT NULL
);

INSERT INTO geofence (name, geofence_type, parent_region_id) VALUES
('Global', 'Global', 0),
('Tajikistan', 'Country', 1),
('Dushanbe', 'City', 2),
('Tursunzade', 'City', 2);