CREATE TABLE IF NOT EXISTS jobs (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  description TEXT NOT NULL DEFAULT '', -- what you'd work on, what you get out of them
  filled BOOLEAN NOT NULL DEFAULT FALSE,
  experience_level TEXT NOT NULL DEFAULT 'junior',
  leadership_work_mentioned BOOLEAN NOT NULL DEFAULT FALSE,
  strictly_rust BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS companies (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  space TEXT NOT NULL DEFAULT '',
  profile TEXT NOT NULL DEFAULT '',
  startup bool,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

ALTER TABLE jobs ADD COLUMN company_id INTEGER REFERENCES companies(id) NOT NULL;