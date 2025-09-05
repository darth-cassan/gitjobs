INSERT INTO "user" (user_id, auth_hash, created_at, email, email_verified, name, username, password, moderator)
VALUES ('f39a95c8-9903-4537-8873-2d81bfb86b35', gen_random_bytes(32), '2025-08-25 08:43:11.605766+02', 'test@t.com', true, 'test', 'test', '$argon2id$v=19$m=19456,t=2,p=1$vUCLsb/lDAepJiWB7VSFNw$yAYeJVIKW0gK3cOJAnpiV9H5uPZDATJh13fDWGivjZM', true);

INSERT INTO employer (employer_id, company, created_at, description, public)
VALUES ('18fff2d7-c794-4130-85e4-76b9d7c60b72', 'Test Inc.', '2025-08-25 09:20:05.88454+02', 'test', false);

INSERT INTO employer_team (user_id, employer_id, approved)
VALUES ('f39a95c8-9903-4537-8873-2d81bfb86b35', '18fff2d7-c794-4130-85e4-76b9d7c60b72', true);

INSERT INTO job (employer_id, title, description, kind, seniority, workplace, status, salary, salary_currency, salary_period, skills, published_at) VALUES
('18fff2d7-c794-4130-85e4-76b9d7c60b72', 'Frontend Developer', 'React expert', 'full-time', 'senior', 'remote', 'published', 120000, 'USD', 'year', '{"React", "TypeScript", "JavaScript"}', CURRENT_TIMESTAMP),
('18fff2d7-c794-4130-85e4-76b9d7c60b72', 'Backend Developer', 'Node.js expert', 'full-time', 'senior', 'hybrid', 'published', 130000, 'USD', 'year', '{"Node.js", "PostgreSQL", "REST"}', CURRENT_TIMESTAMP),
('18fff2d7-c794-4130-85e4-76b9d7c60b72', 'DevOps Engineer', 'Kubernetes expert', 'full-time', 'lead', 'on-site', 'published', 150000, 'USD', 'year', '{"Kubernetes", "Docker", "AWS"}', CURRENT_TIMESTAMP),
('18fff2d7-c794-4130-85e4-76b9d7c60b72', 'Data Scientist', 'Python expert', 'part-time', 'mid', 'remote', 'published', 80000, 'USD', 'year', '{"Python", "Pandas", "scikit-learn"}', CURRENT_TIMESTAMP),
('18fff2d7-c794-4130-85e4-76b9d7c60b72', 'UI/UX Designer', 'Figma expert', 'contractor', 'junior', 'remote', 'published', 60000, 'USD', 'year', '{"Figma", "UI", "UX"}', CURRENT_TIMESTAMP),
('18fff2d7-c794-4130-85e4-76b9d7c60b72', 'Software Engineer in Test', 'Playwright expert', 'full-time', 'mid', 'hybrid', 'published', 110000, 'USD', 'year', '{"Playwright", "TypeScript", "CI/CD"}', CURRENT_TIMESTAMP),
('18fff2d7-c794-4130-85e4-76b9d7c60b72', 'Product Manager', 'Agile expert', 'full-time', 'senior', 'on-site', 'published', 140000, 'USD', 'year', '{"Agile", "Scrum", "Jira"}', CURRENT_TIMESTAMP),
('18fff2d7-c794-4130-85e4-76b9d7c60b72', 'Mobile Developer', 'React Native expert', 'internship', 'entry', 'remote', 'published', 40000, 'USD', 'year', '{"React Native", "iOS", "Android"}', CURRENT_TIMESTAMP),
('18fff2d7-c794-4130-85e4-76b9d7c60b72', 'Full-stack Developer', 'Ruby on Rails expert', 'full-time', 'mid', 'hybrid', 'published', 115000, 'USD', 'year', '{"Ruby on Rails", "PostgreSQL", "React"}', CURRENT_TIMESTAMP),
('18fff2d7-c794-4130-85e4-76b9d7c60b72', 'Security Engineer', 'Cybersecurity expert', 'full-time', 'lead', 'on-site', 'published', 160000, 'USD', 'year', '{"Cybersecurity", "Penetration Testing", "CISSP"}', CURRENT_TIMESTAMP);
