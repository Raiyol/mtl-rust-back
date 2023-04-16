-- Your SQL goes here
CREATE TABLE IF NOT EXISTS novels (
                                      id INT(20) UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
                                      url VARCHAR(50) NOT NULL,
                                      name VARCHAR(50) NOT NULL,
                                      cn_name VARCHAR(30) NOT NULL,
                                      author VARCHAR(20) DEFAULT NULL,
                                      summary TEXT,
                                      img VARCHAR(50) DEFAULT NULL,
                                      date DATETIME DEFAULT UTC_TIMESTAMP,
                                      completed TINYINT NOT NULL,
                                      INDEX(url)
) DEFAULT CHARSET = utf8;
CREATE TABLE IF NOT EXISTS genres (
                                      id INT(20) UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
                                      name varchar(30) NOT NULL
) DEFAULT CHARSET = utf8;
CREATE TABLE IF NOT EXISTS novels_genres (
                                             id_novel int(20) UNSIGNED NOT NULL,
                                             id_genre int(20) UNSIGNED NOT NULL,
                                             PRIMARY KEY (id_novel, id_genre),
                                             FOREIGN KEY(id_novel) REFERENCES novels (id),
                                             FOREIGN KEY(id_genre) REFERENCES genres (id)
) DEFAULT CHARSET = utf8;
CREATE TABLE IF NOT EXISTS chapters (
                                        id int(20) UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
                                        id_novel int(20) UNSIGNED NOT NULL,
                                        chapter int(10) UNSIGNED NOT NULL,
                                        date datetime DEFAULT UTC_TIMESTAMP,
                                        title_en varchar(255) DEFAULT NULL,
                                        title_cn varchar(255) DEFAULT NULL,
                                        content json DEFAULT NULL,
                                        dict json DEFAULT NULL,
                                        INDEX(chapter),
                                        FOREIGN KEY(id_novel) REFERENCES novels (id)
) DEFAULT CHARSET = utf8;
CREATE TABLE IF NOT EXISTS users (
                                     id int(20) UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
                                     name varchar(16) NOT NULL,
                                     email varchar(255) NOT NULL,
                                     pwd varchar(60) NOT NULL,
                                     confirmed tinyint NOT NULL,
                                     role varchar(5),
                                     pfp varchar(32) NOT NULL,
                                     date datetime DEFAULT UTC_TIMESTAMP
) DEFAULT CHARSET = utf8;
CREATE TABLE IF NOT EXISTS bookmarks (
                                         id_user int(20) UNSIGNED NOT NULL,
                                         id_novel int(20) UNSIGNED NOT NULL,
                                         chapter int(10) UNSIGNED NOT NULL,
                                         PRIMARY KEY(id_user, id_novel),
                                         FOREIGN KEY(id_user) REFERENCES users (id),
                                         FOREIGN KEY(id_novel) REFERENCES novels (id)
) DEFAULT CHARSET = utf8;
CREATE TABLE IF NOT EXISTS reviews (
                                       id int(20) UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
                                       id_novel int(20) UNSIGNED NOT NULL,
                                       id_user int(20) UNSIGNED NOT NULL,
                                       rate int(1) DEFAULT NULL,
                                       review text DEFAULT NULL,
                                       date datetime DEFAULT UTC_TIMESTAMP,
                                       FOREIGN KEY(id_user) REFERENCES users (id),
                                       FOREIGN KEY(id_novel) REFERENCES novels (id),
                                       UNIQUE KEY(id_user, id_novel)
) DEFAULT CHARSET = utf8;
CREATE TABLE IF NOT EXISTS likes (
                                     id_review int(20) UNSIGNED NOT NULL,
                                     id_user int(20) UNSIGNED NOT NULL,
                                     PRIMARY KEY (id_review, id_user),
                                     FOREIGN KEY(id_review) REFERENCES reviews (id),
                                     FOREIGN KEY(id_user) REFERENCES users (id)
) DEFAULT CHARSET = utf8;
CREATE TABLE IF NOT EXISTS comments (
                                        id int(20) UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
                                        id_chapter int(20) UNSIGNED NOT NULL REFERENCES chapters (id),
                                        id_user int(20) UNSIGNED NOT NULL REFERENCES users (id),
                                        `comment` varchar(2048) NOT NULL,
                                        date datetime DEFAULT UTC_TIMESTAMP,
                                        FOREIGN KEY(id_chapter) REFERENCES chapters (id),
                                        FOREIGN KEY(id_user) REFERENCES users (id)
) DEFAULT CHARSET = utf8;
