-- Your SQL goes here
CREATE TABLE karat (
  id bigserial NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  gold_purity int NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMP with time zone DEFAULT NULL
);

INSERT INTO karat (name, description, gold_purity) VALUES
('9K', 'This means the gold is 37.5% pure, or 9K. In the US, the minimum standard for gold is 10K', 375), 
('14K', 'This means that the gold is 58.5% OR 58.3% pure, or 14K', 585), 
('18K', 'This means that the gold is 75.0% pure, or 18K. Much more pure than 14K, still has good strength with a wonderful balance in purity', 750),
('24K', 'This means the gold is 99.9% pure, or 24K. This is the purest that you can buy, and although purity can be up to six nines fine, or 999.999, it’s highly rare to find it so pure. Such fineness in gold was last refined in the 1950s by the Perth Mint in Australia', 999);