-- The naming scheme is very Frenglish.  Goal is to have as much code
-- in English as possible, but translating terms from the national
-- teaching programme will make things more confusing to French
-- speakers without improving clarity for others.

-- Users of the service.
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email text NOT NULL UNIQUE,
  display_name TEXT,
  password TEXT NOT NULL,
  password_is_temporary bool DEFAULT false
  );

-- Philosophes.
CREATE TABLE authors (
  id SERIAL PRIMARY KEY,
  slug VARCHAR UNIQUE NOT NULL,
  -- Classic, BibTeX-style five components name.
  name_pre VARCHAR NULL,
  name_first VARCHAR NULL,
  name_von VARCHAR NULL,
  name_last VARCHAR NULL,
  name_suff VARCHAR NULL,
  birth INTEGER NULL,
  death INTEGER NULL,
  official BOOL
  );

-- Les textes. C'est évidemment la table principale de Theuth 1.
CREATE TABLE texts (
  id SERIAL PRIMARY KEY,
  slug VARCHAR UNIQUE NOT NULL,
  title VARCHAR NOT NULL,
  intro TEXT,
  body TEXT NOT NULL,
  -- Pseudo author. If provided, will be used instead of the source's
  -- author.  Useful for secondary source (eg, Aristotle on
  -- Pythagoras should be filed under Pythagoras)
  pseudo_author integer NULL,
  FOREIGN KEY (pseudo_author) REFERENCES authors(id),

  tsource serial,
  translator integer NULL,
  FOREIGN KEY (translator) REFERENCES authors(id),
  publisher text null,
  date integer null
  );

-- Abstract books and articles.
CREATE TABLE sources (
  id SERIAL PRIMARY KEY,
  author serial,
  FOREIGN KEY (author) REFERENCES authors(id),
  title text NOT NULL,
  date integer NULL
)
