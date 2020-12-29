CREATE TABLE users (
  id SERIAL UNIQUE PRIMARY KEY,
  display_name VARCHAR (256) NOT NULL,
  email VARCHAR (256) NOT NULL UNIQUE,
  password TEXT NOT NULL,
  active BOOL NOT NULL default false,
  challenge_code VARCHAR(256) NOT NULL,
  last_login DATE NULL,
  creation_time TIMESTAMP NOT NULL default NOW()
);

CREATE TABLE sessions (
  id SERIAL UNIQUE PRIMARY KEY,
  secret VARCHAR (64) NOT NULL UNIQUE,
  sess_user int NOT NULL,
  acctime TIMESTAMP NOT NULL default NOW(),

  FOREIGN KEY (sess_user) REFERENCES users (id)

  );
