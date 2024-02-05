CREATE TYPE user_role AS ENUM ('administrator', 'committee_operator', 'committee', 'general');
CREATE TYPE user_category AS ENUM ('undergraduate_student', 'graduate_student', 'academic_staff');

CREATE TABLE users (
  id TEXT PRIMARY KEY, -- Firebase AuthのためにUUIDでなくTEXT

  name TEXT NOT NULL,
  kana_name TEXT NOT NULL,

  email TEXT NOT NULL UNIQUE,
  phone_number TEXT NOT NULL UNIQUE,
  role user_role NOT NULL DEFAULT 'general',
  category user_category NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ DEFAULT NULL
);

CREATE TYPE project_status AS ENUM ('not_verified', 'verified');

CREATE TABLE projects (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  index INTEGER NOT NULL GENERATED ALWAYS AS IDENTITY UNIQUE,
  status project_status NOT NULL,

  title TEXT NOT NULL,
  kana_title TEXT NOT NULL,
  description TEXT NOT NULL,

  group_name TEXT NOT NULL,
  kana_group_name TEXT NOT NULL,

  attributes INTEGER NOT NULL DEFAULT 0,

  owner_id TEXT NOT NULL REFERENCES users(id),
  sub_owner_id TEXT NOT NULL REFERENCES users(id),

  remarks TEXT DEFAULT NULL, -- 削除理由など

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ DEFAULT NULL
);

CREATE TABLE news (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

  title TEXT NOT NULL,
  body TEXT NOT NULL,
  categories INTEGER NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMPTZ DEFAULT NULL
);

CREATE TABLE news_attachments (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  news_id UUID NOT NULL REFERENCES news(id),

  url TEXT NOT NULL,

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
// TRIGGERS (news_attachments)
*/
CREATE TRIGGER refresh_news_attachments_updated_at_step1
    BEFORE UPDATE ON news_attachments FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step1();
CREATE TRIGGER refresh_news_attachments_updated_at_step2
    BEFORE UPDATE OF updated_at ON news_attachments FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step2();
CREATE TRIGGER refresh_news_attachments_updated_at_step3
    BEFORE UPDATE ON news_attachments FOR EACH ROW
    EXECUTE PROCEDURE refresh_updated_at_step3();