-- Your SQL goes here
CREATE TABLE contest_types (
    id INTEGER PRIMARY KEY NOT NULL,
    identifier VARCHAR NOT NULL
);

CREATE TABLE contest_effects (
    id INTEGER PRIMARY KEY NOT NULL,
    appeal INTEGER NOT NULL,
    jam INTEGER NOT NULL
);

-- 插入 contest_types 数据
INSERT INTO contest_types (id, identifier) VALUES
    (1, 'cool'),
    (2, 'beauty'),
    (3, 'cute'),
    (4, 'smart'),
    (5, 'tough');

-- 插入 contest_effects 数据
INSERT INTO contest_effects (id, appeal, jam) VALUES
    (1, 4, 0), (2, 3, 0), (3, 6, 0), (4, 1, 4), (5, 1, 3),
    (6, 4, 4), (7, 8, 0), (8, 2, 2), (9, 2, 3), (10, 2, 1),
    (11, 1, 0), (12, 2, 0), (13, 1, 0), (14, 2, 1), (15, 1, 0),
    (16, 2, 0), (17, 3, 0), (18, 2, 0), (19, 1, 0), (20, 1, 0),
    (21, 3, 0), (22, 3, 0), (23, 2, 1), (24, 3, 0), (25, 1, 0),
    (26, 1, 0), (27, 2, 0), (28, 2, 0), (29, 1, 0), (30, 3, 0),
    (31, 3, 0), (32, 1, 0), (33, 3, 0);
