CREATE TYPE news_state AS ENUM ('draft', 'scheduled', 'published');
ALTER TABLE news ADD COLUMN state news_state NOT NULL DEFAULT 'draft';
ALTER TABLE news ADD COLUMN scheduled_at TIMESTAMPTZ DEFAULT NULL;
