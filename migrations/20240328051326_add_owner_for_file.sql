ALTER TABLE files ADD COLUMN owner_project UUID REFERENCES projects(id);
