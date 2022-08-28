Use myDB;

DROP TABLE IF EXISTS `product`;
DROP TABLE IF EXISTS `user`;

CREATE TABLE `user` (
  `id` varchar(255) NOT NULL,
  `name` varchar(255) NOT NULL,
  `email` varchar(255) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `email` (`email`)
);

CREATE TABLE `product` (
  `id` varchar(255) NOT NULL,
  `user_id` varchar(255) NOT NULL,
  `name` varchar(255) NOT NULL,
  `price` decimal(10,0) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `product_fk0` (`user_id`),
  CONSTRAINT `product_fk0` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`) ON DELETE CASCADE
);

