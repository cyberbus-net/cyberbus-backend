-- ADD column config for local_site table
ALTER TABLE local_site
    ADD COLUMN require_invite_code boolean DEFAULT FALSE;
