ALTER TYPE user_role RENAME VALUE 'committee' TO 'committee_viewer';
ALTER TYPE user_role ADD VALUE 'committee_editor' AFTER 'committee_operator';
ALTER TYPE user_role ADD VALUE 'committee_drafter' AFTER 'committee_editor';
