-- -------------------------------------
-- user
-- -------------------------------------
INSERT INTO mydb.`user` (id, name, email) VALUES('1', 'tommy', 'tommy@email.com');
INSERT INTO mydb.`user` (id, name, email) VALUES('2', 'bob', 'bob@email.com');
INSERT INTO mydb.`user` (id, name, email) VALUES('3', 'eddy', 'eddy@email.com');

-- -------------------------------------
-- product
-- -------------------------------------
INSERT INTO mydb.product (id, user_id, name, price) VALUES('1', '1', 'TV', 1000);
INSERT INTO mydb.product (id, user_id, name, price) VALUES('2', '2', 'Laptop', 1100);
INSERT INTO mydb.product (id, user_id, name, price) VALUES('3', '3', 'Car', 50000);
