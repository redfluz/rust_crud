CREATE DATABASE IF NOT EXISTS user_crud
    CHARACTER SET utf8mb4
    COLLATE utf8mb4_unicode_ci;

USE user_crud;

CREATE TABLE IF NOT EXISTS users (
                                     id INT UNSIGNED NOT NULL AUTO_INCREMENT,
                                     name VARCHAR(255) NOT NULL,
    age INT UNSIGNED NOT NULL,
    PRIMARY KEY (id)
    ) ENGINE=InnoDB
    DEFAULT CHARSET=utf8mb4
    COLLATE=utf8mb4_unicode_ci;