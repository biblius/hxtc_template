CREATE TABLE users(
  id uuid UNIQUE DEFAULT uuid_generate_v4() NOT NULL,
  username VARCHAR(32) NOT NULL,
  "password" VARCHAR(255),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  CONSTRAINT pk_users PRIMARY KEY (id)
);

-- Diesel helper to automatically adjust the 'updated_at' field
SELECT diesel_manage_updated_at('users');