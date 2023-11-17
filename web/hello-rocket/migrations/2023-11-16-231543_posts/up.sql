CREATE TABLE posts (
       id          SERIAL     NOT NULL  PRIMARY KEY,
       title       VARCHAR    NOT NULL,
       body        TEXT       NOT NULL,
       published   BOOLEAN    NOT NULL  DEFAULT FALSE
);