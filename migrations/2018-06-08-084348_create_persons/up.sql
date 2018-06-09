-- Your SQL goes here

CREATE TABLE persons (
  id SERIAL NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  vote BIGINT NOT NULL DEFAULT 0,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (name)
);

INSERT INTO persons (name) VALUES
  ('Tanaka'),
  ('Takahashi'),
  ('Yamada');
