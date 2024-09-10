-- ADD column trophy_case for local_user table
ALTER TABLE local_user
    ADD COLUMN trophy_case JSONB;
