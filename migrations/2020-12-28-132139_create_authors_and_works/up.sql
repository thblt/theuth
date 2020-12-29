CREATE TABLE authors (
  -- Authors.
  id SERIAL UNIQUE PRIMARY KEY,
  -- BibTeX-style five components name.
  name_prefix VARCHAR NULL,
  name_first VARCHAR NULL,
  name_von VARCHAR NULL,
  name_last VARCHAR NULL,
  name_suffix VARCHAR NULL,
  birth INTEGER NULL,
  death INTEGER NULL,
  au_programme BOOL NOT NULL DEFAULT false,
  active BOOL NOT NULL default true
  );

CREATE TABLE works (
  id SERIAL UNIQUE PRIMARY KEY,
  title TEXT NOT NULL,
  containing_title TEXT default null,
  author int,
  FOREIGN KEY (author) REFERENCES authors (id),
  active BOOL NOT NULL default true
);
