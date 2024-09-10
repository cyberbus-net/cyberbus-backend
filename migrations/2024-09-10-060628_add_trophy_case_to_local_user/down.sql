-- This file should undo anything in `up.sql`
ALTER TABLE local_user
    DROP COLUMN trophy_case;
