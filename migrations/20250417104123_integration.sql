CREATE TYPE user_role AS ENUM ('administrator', 'committee_operator', 'committee_editor', 'committee_drafter', 'committee_viewer', 'general');
-- CREATE TYPE user_category AS ENUM ('undergraduate_student', 'graduate_student', 'academic_staff');

CREATE TABLE users (
  id TEXT PRIMARY KEY, -- Firebase AuthのためにUUIDでなくTEXT

  name TEXT NOT NULL,
  kana_name TEXT NOT NULL,

  email TEXT NOT NULL UNIQUE,
  phone_number TEXT NOT NULL UNIQUE,
  role user_role NOT NULL DEFAULT 'general',
  -- category user_category NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ DEFAULT NULL
);

CREATE TYPE project_category AS ENUM ('general', 'foods_with_kitchen', 'foods_without_kitchen', 'foods_without_cooking', 'stage_1a', 'stage_university_hall', 'stage_united');

CREATE TABLE projects (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  index INTEGER GENERATED ALWAYS AS IDENTITY UNIQUE,

  title TEXT NOT NULL,
  kana_title TEXT NOT NULL,
  -- description TEXT NOT NULL,

  group_name TEXT NOT NULL,
  kana_group_name TEXT NOT NULL,

  category project_category NOT NULL,
  attributes INTEGER NOT NULL DEFAULT 0,

  owner_id TEXT NOT NULL REFERENCES users(id),
  sub_owner_id TEXT DEFAULT NULL REFERENCES users(id),

  remarks TEXT DEFAULT NULL, -- 削除理由など
  location_id TEXT DEFAULT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ DEFAULT NULL
);

CREATE TYPE news_state AS ENUM ('draft', 'scheduled', 'published');

CREATE TABLE news (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

  state news_state NOT NULL DEFAULT 'draft',
  title TEXT NOT NULL,
  body TEXT NOT NULL,
  categories INTEGER NOT NULL,
  attributes INTEGER NOT NULL,
  attachments UUID[] NOT NULL DEFAULT '{}',
  scheduled_at TIMESTAMPTZ DEFAULT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ DEFAULT NULL
);

CREATE TABLE files (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  -- news_id UUID NOT NULL REFERENCES news(id),
  name TEXT NOT NULL,
  owner_project UUID REFERENCES projects(id),

  url TEXT NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ DEFAULT NULL
);

CREATE TYPE invitation_position AS ENUM ('owner', 'sub_owner');

CREATE TABLE invitations (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

  inviter TEXT NOT NULL REFERENCES users(id),
  project_id UUID NOT NULL REFERENCES projects(id),
  position invitation_position NOT NULL,
  used_by TEXT DEFAULT NULL REFERENCES users(id),

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ DEFAULT NULL
);

/*
// TRIGGERS
*/
CREATE FUNCTION refresh_updated_at_step1() RETURNS trigger AS
$$
BEGIN
  IF NEW.updated_at = OLD.updated_at THEN
    NEW.updated_at := NULL;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
    
CREATE FUNCTION refresh_updated_at_step2() RETURNS trigger AS
$$
BEGIN
  IF NEW.updated_at IS NULL THEN
    NEW.updated_at := OLD.updated_at;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION refresh_updated_at_step3() RETURNS trigger AS
$$
BEGIN
  IF NEW.updated_at IS NULL THEN
    NEW.updated_at := CURRENT_TIMESTAMP;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

/*
// TRIGGERS (users)
*/
CREATE TRIGGER refresh_users_updated_at_step1
    BEFORE UPDATE ON users FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step1();
CREATE TRIGGER refresh_users_updated_at_step2
    BEFORE UPDATE OF updated_at ON users FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step2();
CREATE TRIGGER refresh_users_updated_at_step3
    BEFORE UPDATE ON users FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step3();

/*
// TRIGGERS (projects)
*/
CREATE TRIGGER refresh_projects_updated_at_step1
    BEFORE UPDATE ON projects FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step1();
CREATE TRIGGER refresh_projects_updated_at_step2
    BEFORE UPDATE OF updated_at ON projects FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step2();
CREATE TRIGGER refresh_projects_updated_at_step3
    BEFORE UPDATE ON projects FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step3();

/*
// TRIGGERS (news)
*/
CREATE TRIGGER refresh_news_updated_at_step1
    BEFORE UPDATE ON news FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step1();
CREATE TRIGGER refresh_news_updated_at_step2
    BEFORE UPDATE OF updated_at ON news FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step2();
CREATE TRIGGER refresh_news_updated_at_step3
    BEFORE UPDATE ON news FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step3();

/*
// TRIGGERS (files)
*/
CREATE TRIGGER refresh_files_updated_at_step1
    BEFORE UPDATE ON files FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step1();
CREATE TRIGGER refresh_files_updated_at_step2
    BEFORE UPDATE OF updated_at ON files FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step2();
CREATE TRIGGER refresh_files_updated_at_step3
    BEFORE UPDATE ON files FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step3();

/*
// TRIGGERS (invitations)
*/
CREATE TRIGGER refresh_invitations_updated_at_step1
    BEFORE UPDATE ON invitations FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step1();
CREATE TRIGGER refresh_invitations_updated_at_step2
    BEFORE UPDATE OF updated_at ON invitations FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step2();
CREATE TRIGGER refresh_invitations_updated_at_step3
    BEFORE UPDATE ON invitations FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step3();
