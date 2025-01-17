create extension pgcrypto;
create extension postgis;

create table job_board (
    job_board_id uuid primary key default gen_random_uuid(),

    active boolean not null default true,
    created_at timestamptz not null default current_timestamp,
    description text not null check (description <> ''),
    display_name text not null unique check (display_name <> ''),
    header_logo_url text not null check (header_logo_url <> ''),
    host text not null unique check (host <> ''),
    name text not null unique check (name <> ''),
    theme jsonb not null,
    title text not null check (title <> ''),

    about_intro text check (about_intro <> ''),
    extra_links jsonb,
    footer_logo_url text check (footer_logo_url <> '')
);

create table location (
    location_id uuid primary key,

    city text not null check (city <> ''),
    country text not null check (country <> ''),

    coordinates geography(point, 4326),
    state text check (state <> '')
);

create index location_city_idx on location (city);
create index location_coordinates_idx on location using gist (coordinates);
create index location_country_idx on location (country);

create table profile (
    profile_id uuid primary key default gen_random_uuid(),
    job_board_id uuid  not null references job_board (job_board_id),
    location_id uuid references location (location_id),

    email text not null unique check (email <> ''),
    first_name text not null check (first_name <> ''),
    last_name text not null check (last_name <> ''),
    public boolean not null default false,
    summary text not null check (summary <> ''),

    facebook_url text check (facebook_url <> ''),
    github_url text check (github_url <> ''),
    linkedin_url text check (linkedin_url <> ''),
    open_to_relocation boolean,
    open_to_remote boolean,
    phone text check (phone <> ''),
    photo_url text check (photo_url <> ''),
    resume_blob bytea,
    resume_filename text check (resume_filename <> ''),
    skills text[],
    twitter_url text check (twitter_url <> ''),
    website_url text check (website_url <> '')
);

create index profile_job_board_id_idx on profile (job_board_id);
create index profile_location_id_idx on profile (location_id);

create table profile_certification (
    profile_certification_id uuid primary key default gen_random_uuid(),
    profile_id uuid not null references profile (profile_id),

    description text not null check (description <> ''),
    end_date date not null,
    provider text not null check (provider <> ''),
    start_date date not null,
    title text not null check (title <> '')
);

create index profile_certification_profile_id_idx on profile_certification (profile_id);

create table profile_education (
    profile_education_id uuid primary key default gen_random_uuid(),
    profile_id uuid not null references profile (profile_id),

    description text not null check (description <> ''),
    educational_institution text not null check (educational_institution <> ''),
    end_date date not null,
    start_date date not null,
    title text not null check (title <> '')
);

create index profile_education_profile_id_idx on profile_education (profile_id);

create table profile_employment (
    profile_employment_id uuid primary key default gen_random_uuid(),
    profile_id uuid not null references profile (profile_id),

    company text not null check (company <> ''),
    current boolean not null default false,
    description text not null check (description <> ''),
    end_date date not null,
    start_date date not null,
    title text not null check (title <> '')
);

create index profile_employment_profile_id_idx on profile_employment (profile_id);

create table profile_project (
    profile_project_id uuid primary key default gen_random_uuid(),
    profile_id uuid not null references profile (profile_id),

    description text not null check (description <> ''),
    title text not null check (title <> ''),
    url text not null check (url <> ''),

    source_url text check (source_url <> '')
);

create index profile_project_profile_id_idx on profile_project (profile_id);

create table employer_tier (
    employer_tier_id uuid primary key default gen_random_uuid(),
    job_board_id uuid  not null references job_board (job_board_id),

    name text not null unique check (name <> ''),
    highlight boolean not null default false,
    priority int not null default 0
);

create index tier_job_board_id_idx on employer_tier (job_board_id);

create table employer (
    employer_id uuid primary key default gen_random_uuid(),
    employer_tier_id uuid not null references employer_tier (employer_tier_id),
    location_id uuid references location (location_id),

    company text not null check (company <> ''),
    created_at timestamptz not null default current_timestamp,
    description text not null check (description <> ''),
    public boolean not null default false,

    logo_url text check (logo_url <> ''),
    updated_at timestamptz,
    website_url text check (website_url <> '')
);

create index employer_employer_tier_id_idx on employer (employer_tier_id);
create index employer_location_id_idx on employer (location_id);

create table job_type (
    job_type_id uuid primary key default gen_random_uuid(),
    name text not null unique check (name <> '')
);

insert into job_type (name) values ('Full Time');
insert into job_type (name) values ('Part Time');
insert into job_type (name) values ('Contractor');
insert into job_type (name) values ('Internship');

create table workplace (
    workplace_id uuid primary key default gen_random_uuid(),
    name text not null unique check (name <> '')
);

insert into workplace (name) values ('Hybrid');
insert into workplace (name) values ('On Site');
insert into workplace (name) values ('Remote');

create table job (
    job_id uuid primary key default gen_random_uuid(),
    employer_id uuid not null references employer (employer_id),
    job_type_id uuid not null references job_type (job_type_id),
    workplace_id uuid not null references workplace (workplace_id),
    location_id uuid references location (location_id),

    created_at timestamptz not null default current_timestamp,
    title text not null check (title <> ''),
    description text not null check (description <> ''),

    apply_url text check (apply_url <> ''),
    apply_instructions text check (apply_instructions <> ''),
    benefits text[],
    expires_at timestamptz,
    location geography(point, 4326),
    open_source int check (open_source >= 0 and open_source <= 100),
    published_at timestamptz,
    salary bigint check (salary >= 0),
    salary_currency text check (salary_currency <> ''),
    salary_max bigint check (salary_max >= 0),
    salary_min bigint check (salary_min >= 0),
    salary_timeframe text check (salary_timeframe <> ''),
    skills text[],
    updated_at timestamptz,
    upstream_commitment int check (upstream_commitment >= 0 and upstream_commitment <= 100)
);

create index job_employer_id_idx on job (employer_id);
create index job_job_type_id_idx on job (job_type_id);
create index job_workplace_id_idx on job (workplace_id);
create index job_location_id_idx on job (location_id);

create table applicant (
    applicant_id uuid primary key default gen_random_uuid(),
    profile_id uuid not null references profile (profile_id),
    job_id uuid not null references job (job_id),

    cover_letter text not null check (cover_letter <> ''),
    created_at timestamptz not null default current_timestamp,

    updated_at timestamptz
);

create index applicant_profile_id_idx on applicant (profile_id);
create index applicant_job_id_idx on applicant (job_id);

create table faq (
    faq_id uuid primary key default gen_random_uuid(),
    job_board_id uuid  not null references job_board (job_board_id),

    answer text not null check (answer <> ''),
    question text not null check (question <> '')
);

create index faq_job_board_id_idx on faq (job_board_id);
