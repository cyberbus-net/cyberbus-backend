-- This file should undo anything in `up.sql`
ALTER TABLE local_site
    DROP COLUMN require_invite_code;
