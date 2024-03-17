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
