CREATE TABLE users (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  email TEXT,
  description TEXT,
);

CREATE TABLE categories (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT
);

CREATE TABLE series (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT
);

CREATE TABLE posts (
  id UUID PRIMARY KEY,
  status INTEGER NOT NULL,
  timestamp TIMESTAMP NOT NULL,
  url TEXT,
  title TEXT NOT NULL,
  abstract TEXT NOT NULL,
  content TEXT NOT NULL,
  file TEXT,
  series UUID REFERENCES series(id) ON DELETE RESTRICT,
  category UUID NOT NULL REFERENCES categories(id) ON DELETE RESTRICT,
  commentable INTEGER NOT NULL
);

CREATE INDEX posts_timestamp ON posts (
  timestamp DESC,
  status
);

CREATE INDEX posts_series ON posts (
  series,
  timestamp DESC,
  status
);

CREATE INDEX posts_category ON posts (
  category,
  timestamp DESC,
  status
);

CREATE INDEX posts_title ON posts USING gin (
  to_tsvector('simple', title)
);

CREATE INDEX posts_abstract ON posts USING gin (
  to_tsvector('simple', abstract)
);

CREATE INDEX posts_content ON posts USING gin (
  to_tsvector('simple', content)
);

CREATE TABLE post_tags (
  post UUID NOT NULL REFERENCES posts(id) ON DELETE RESTRICT,
  tag TEXT NOT NULL,
  PRIMARY KEY (post, tag)
);

CREATE INDEX post_tags_post ON post_tags (
  post
);

CREATE INDEX post_tags_tag ON post_tags (
  tag
);
