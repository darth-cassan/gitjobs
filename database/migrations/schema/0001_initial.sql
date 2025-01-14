create extension pgcrypto;

create table board (
    board_id uuid primary key default gen_random_uuid(),
    active boolean default true not null,
    created_at timestamptz default current_timestamp not null,
    description text not null check (description <> ''),
    display_name text not null unique check (display_name <> ''),
    header_logo_url text not null check (header_logo_url <> ''),
    host text not null unique check (host <> ''),
    name text not null unique check (name <> ''),
    theme jsonb not null,
    title text not null check (title <> ''),

    extra_links jsonb,
    footer_logo_url text check (footer_logo_url <> '')
);
