-- Your SQL goes here
-- Description: This file contains the SQL statements for creating the database schema.

-- Extensions 
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Types
CREATE TYPE page_modes AS ENUM ('read', 'write', 'admin');
    

--
-- Security and Authentication Section
--
CREATE TABLE IF NOT EXISTS users(
    id SERIAL PRIMARY KEY,
    name VARCHAR(256) NOT NULL UNIQUE,
    email_address VARCHAR(256) NOT NULL UNIQUE,
    password VARCHAR(256) NOT NULL, --  COMMENT 'hashed password'
    role VARCHAR(64) NOT NULL, --  COMMENT 'role of the user e.g. admin, user, etc.'
    profile_id INTEGER,
    company_id INTEGER,
    team_id integer,
    -- metadata
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    updated_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    created_by INTEGER NOT NULL DEFAULT 0,
    updated_by INTEGER NOT NULL DEFAULT 0,
    active BOOL NOT NULL DEFAULT true);

CREATE TABLE IF NOT EXISTS companies(
    id SERIAL PRIMARY KEY,
    name VARCHAR(256) NOT NULL UNIQUE,
    owner_id INTEGER NOT NULL,
    profile_id INTEGER,
    -- metadata
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    updated_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    created_by INTEGER NOT NULL DEFAULT 0,
    updated_by INTEGER NOT NULL DEFAULT 0,
    active BOOL NOT NULL DEFAULT true);

CREATE TABLE IF NOT EXISTS teams(
    id SERIAL PRIMARY KEY,
    name VARCHAR(256) NOT NULL UNIQUE,
    company_id INTEGER NOT NULL,
    owner_id INTEGER NOT NULL,
    profile_id INTEGER,
    -- metadata
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    updated_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    created_by INTEGER NOT NULL DEFAULT 0,
    updated_by INTEGER NOT NULL DEFAULT 0,
    active BOOL NOT NULL DEFAULT true);

CREATE TABLE IF NOT EXISTS teams_user_index (
    id SERIAL PRIMARY KEY,
    team_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    -- metadata
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    updated_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    created_by INTEGER NOT NULL DEFAULT 0,
    updated_by INTEGER NOT NULL DEFAULT 0,
    active BOOL NOT NULL DEFAULT true
);

CREATE TABLE IF NOT EXISTS profile (
    id SERIAL PRIMARY KEY,
    avatar_url VARCHAR(256),
    bio text,
    default_page_id UUID,
    -- metadata
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    updated_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    created_by INTEGER NOT NULL DEFAULT 0,
    updated_by INTEGER NOT NULL DEFAULT 0,
    active BOOL NOT NULL DEFAULT true);

--
-- Pages and Block Section
--
CREATE TABLE IF NOT EXISTS pages (
    id UUID PRIMARY KEY NOT NULL,
    name VARCHAR(255),
    owner_id INTEGER NOT NULL,  
    company_id INTEGER,
    team_id INTEGER,
    parent_page_id UUID NOT NULL,
    -- metadata
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    updated_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    created_by INTEGER NOT NULL DEFAULT 0,
    updated_by INTEGER NOT NULL DEFAULT 0,
    active BOOL NOT NULL DEFAULT true);

CREATE TABLE IF NOT EXISTS page_versions (
    id UUID PRIMARY KEY NOT NULL,
    page_id UUID NOT NULL,
    version integer NOT NULL,
    -- metadata
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    updated_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    created_by INTEGER NOT NULL DEFAULT 0,
    updated_by INTEGER NOT NULL DEFAULT 0,
    active BOOL NOT NULL DEFAULT true,
    -- foreign keys
    FOREIGN KEY (page_id) REFERENCES pages (id)
);

CREATE TABLE IF NOT EXISTS blocks (
    id UUID PRIMARY KEY NOT NULL,
    block_id UUID NOT NULL,
    version integer NOT NULL,
    block_type VARCHAR(64) NOT NULL,
    content text,
        -- metadata
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    updated_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    created_by INTEGER NOT NULL DEFAULT 0,
    updated_by INTEGER NOT NULL DEFAULT 0,
    active BOOL NOT NULL DEFAULT true);

CREATE TABLE IF NOT EXISTS page_block_index (
    id UUID PRIMARY KEY NOT NULL,
    page_version_id UUID NOT NULL,
    display_order integer NOT NULL,
    block_id UUID NOT NULL,
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    FOREIGN KEY (page_version_id) REFERENCES page_versions (id),
    FOREIGN KEY (block_id) REFERENCES blocks (id)
 );

CREATE TABLE IF NOT EXISTS page_permission (
	id SERIAL PRIMARY KEY, 
	page_id UUID, 
	user_id integer, 
	team_id integer, 
	company_id integer, 
	allow_all boolean,
    mode page_modes default 'read',
	created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    updated_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    created_by INTEGER NOT NULL DEFAULT 0,
    updated_by INTEGER NOT NULL DEFAULT 0,
    active BOOL NOT NULL DEFAULT true);
