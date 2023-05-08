CREATE TABLE sessions(
  id uuid UNIQUE DEFAULT uuid_generate_v4() NOT NULL,
  "user_id" uuid NOT NULL,
  csrf uuid UNIQUE NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  expires_at TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '1 HOUR',
  CONSTRAINT pk_sessions PRIMARY KEY (id),
  CONSTRAINT fk_sessions_user_id FOREIGN KEY ("user_id") REFERENCES users(id) ON DELETE CASCADE
);
SELECT diesel_manage_updated_at('sessions');
CREATE INDEX IF NOT EXISTS sessions_user_id ON "sessions" USING BTREE(user_id);