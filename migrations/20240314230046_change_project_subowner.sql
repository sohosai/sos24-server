ALTER TABLE projects
ALTER COLUMN sub_owner_id DROP NOT NULL,
ALTER COLUMN sub_owner_id SET DEFAULT NULL;