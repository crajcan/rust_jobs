CREATE TABLE IF NOT EXISTS jobs (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  description TEXT NOT NULL DEFAULT '', -- what you'd work on, what you get out of them
  filled BOOLEAN NOT NULL DEFAULT FALSE,
  url TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS companies (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  space TEXT NOT NULL DEFAULT '',
  profile TEXT NOT NULL DEFAULT '',
  startup bool DEFAULT false,
  url TEXT,
  open_source_github_org TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

ALTER TABLE jobs ADD COLUMN company_id INTEGER REFERENCES companies(id) NOT NULL;

CREATE TABLE IF NOT EXISTS sneaky_crypto (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

ALTER TABLE companies ADD COLUMN open_source_github_org TEXT;