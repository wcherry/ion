CREATE TABLE IF NOT EXISTS pages
(
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255),
    updated_by VARCHAR(255),
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    deleted boolean DEFAULT false
);

CREATE TABLE IF NOT EXISTS page_details
(
    id SERIAL PRIMARY KEY NOT NULL,
    page_id integer NOT NULL,
    version integer NOT NULL,
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    deleted boolean DEFAULT false,
    FOREIGN KEY (page_id) REFERENCES pages (id)
);

CREATE TABLE IF NOT EXISTS page_details_content
(
    page_details_id integer PRIMARY KEY NOT NULL,
    raw_content text,
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    deleted boolean DEFAULT false
);
