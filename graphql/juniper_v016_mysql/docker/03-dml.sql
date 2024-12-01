-- -------------------------------------
-- user
-- -------------------------------------
INSERT INTO myDB.`user` (id, name, email) VALUES('1', 'tommy', 'tommy@email.com');
INSERT INTO myDB.`user` (id, name, email) VALUES('2', 'bob', 'bob@email.com');
INSERT INTO myDB.`user` (id, name, email) VALUES('3', 'eddy', 'eddy@email.com');

-- -------------------------------------
-- product
-- -------------------------------------
INSERT INTO myDB.product (id, user_id, name, price) VALUES('1', '1', 'TV', 1000);
INSERT INTO myDB.product (id, user_id, name, price) VALUES('2', '2', 'Laptop', 1100);
INSERT INTO myDB.product (id, user_id, name, price) VALUES('3', '3', 'Car', 50000);
