INSERT INTO competition (created_at, name)
VALUES (unixepoch(), 'default competition');

INSERT INTO mosaic_design (name, pixels)
VALUES ('RIT Landscape',
        '["gggggggggggggggggggrgrwwwgyyyy","gggwwwgggggggwwgggggrgwwggyyyy","ggwwwwrgrgggwwwwggwwwggggggyyy","wggwwwgrgwwggwwggwwwwwgggggggg","wwggggggwwwwgggrgrwwwggggggwwg","wggggggggwwgggggrgggggggggwwww","gggggggggggggggggggggggggggwwg","gbbbbbbbbggggbbbbbggbbbbbbbbbg","gboooooobbgggbooobggbooooooobg","gbbobbboobgggbbobbggbobbobbobg","ggbobgboobggggbobgggbbbbobbbbg","yybobbbobbggggbobggggggbobgggg","rrbooooobgggggbobggggggbobgggg","brboooooobggggbobggggggbobggyy","rrbobbbbobggggbobggggggbobyyyy","rrbobrrbobygggbobggggggbobrrrr","rrbobrrbobygggbobggggggbobrrrr","rbbobrrbobbyrbbobbggggbbobbrbb","gboobggboobyybooobggggbooobrbb","gbbbbggbbbbyybbbbbggggbbbbbrrr","ggggggggggyyyygggggggggggrrrrr","ggggbwbggggyrybgggoggggggggggg","ggbwbbbwbgggyyygrggggrgggggogg","bwbbbwbbbwbgyyyggggrgggggggggy","gbbwbbbwbbrgbyyyyggggbggggbyyy","bbgbbwbbrbbggoyybyyyyyyyyyyyry","gbbbgbrbbbrgggyyyybyyyyyyyyygg","bbgbbbbbrbbgggggggggoggggggggo","gbbbgbrbbbrggggggggggggggrgogg","bbgbbbbbrbbgggggggggggbggggggg"]');

INSERT INTO mosaic_design (name, pixels)
VALUES ('Apple',
        '["wwwgggggwwwwwwwwwwwwwwwwwwwwww","wggggggggggwwwwwwwowwwwwwwwwww","wwggggggggggwwwwwooowwwwwwwwww","wwwggggggggggwwwwowwwwwwwwwwww","wwwwwgggggggggwwwowwwwwwwwwwww","wwwwwwwwwwggggwwowwwwwwwwwwwww","wwwwwwwwwwwwwggwowwwwwwwwwwwww","wwwwwwwwwwwwwwwwrrrrrrwwwwwwww","wwwwwwwwworrrrrrrrrrrrrrrwwwww","wwwwwworrrrrrrrrrrrrrrrrrrwwww","wwwwwwrrrrrrrrrrrrrrrrrrrrrwww","wwwwwrrrrrrrrrrorrrrrrrrrrrrww","wwwwwrrrrrrrrrrrrrrrrrrrrrrrww","wwwwwrrrrrrrrrrrrrrrrrrrrrrrrw","wwwwwrrrrrrrrrrrrrrrrrrrrrrrrw","wwwwwrrrrrrrrrrrrrrrrrrrrrrrrw","wwwwwrrrrrrrrrrrrrrrrrrrrrrrow","wwwwwrrrrrrrrrrrrrrrrrrrrrrrww","wwwwwrrrrrrrrrrrrrrrrrrrrrrrww","wwwwwrrrrrrrrrrrrrrrrrrrrrrrww","wwwwwwrrrrrrrrrrrrrrrrrrrrrrww","wwwwwwrrrrrrrrrrrrrrrrrrrrrwww","wwwwwwrrrrrrrrrrrrrrrrrrrrrwww","wwwwwwwrrrrrrrrrrrrrrrrrrrwwww","wwwwwwwrrrrrrrrrrrrrrrrrrrwwww","wwwwwwwwrrrrrrrrrrrrrrrrrwwwww","wwwwwwwworrrrrrrrrrrrrrrrwwwww","wwwwwwwwwrrrrrrrrrrrrrrrwwwwww","wwwwwwwwwwrrrrrrrrrrrrrwwwwwww","wwwwwwwwwwwrrrrwwrrrrwwwwwwwww"]');

INSERT INTO _config (competition, cg_link, mosaic_design)
VALUES (1, 'https://www.youtube.com/watch?v=dQw4w9WgXcQ', 1);

INSERT INTO user (sub, full_name, given_name, family_name, created_at, login_at)
VALUES (0, 'admin admin', 'admin', 'admin', unixepoch(), unixepoch());

INSERT INTO role (name)
VALUES ('admin');

INSERT INTO user_role (user, role)
VALUES (1, 1);

INSERT INTO puzzle
VALUES (1, '3x3');
INSERT INTO puzzle
VALUES (2, '2x2');
INSERT INTO puzzle
VALUES (3, '4x4');
INSERT INTO puzzle
VALUES (4, '5x5');
INSERT INTO puzzle
VALUES (5, '6x6');
INSERT INTO puzzle
VALUES (6, '7x7');
INSERT INTO puzzle
VALUES (7, 'Pyraminx');
INSERT INTO puzzle
VALUES (8, 'Square 1');
INSERT INTO puzzle
VALUES (9, 'Megaminx');
INSERT INTO puzzle
VALUES (10, 'Clock');
INSERT INTO puzzle
VALUES (11, 'Skewb');

INSERT INTO mosaic_tile
VALUES (1, 0, 0, NULL);
INSERT INTO mosaic_tile
VALUES (2, 0, 1, NULL);
INSERT INTO mosaic_tile
VALUES (3, 0, 2, NULL);
INSERT INTO mosaic_tile
VALUES (4, 0, 3, NULL);
INSERT INTO mosaic_tile
VALUES (5, 0, 4, NULL);
INSERT INTO mosaic_tile
VALUES (6, 0, 5, NULL);
INSERT INTO mosaic_tile
VALUES (7, 0, 6, NULL);
INSERT INTO mosaic_tile
VALUES (8, 0, 7, NULL);
INSERT INTO mosaic_tile
VALUES (9, 0, 8, NULL);
INSERT INTO mosaic_tile
VALUES (10, 0, 9, NULL);
INSERT INTO mosaic_tile
VALUES (11, 1, 0, NULL);
INSERT INTO mosaic_tile
VALUES (12, 1, 1, NULL);
INSERT INTO mosaic_tile
VALUES (13, 1, 2, NULL);
INSERT INTO mosaic_tile
VALUES (14, 1, 3, NULL);
INSERT INTO mosaic_tile
VALUES (15, 1, 4, NULL);
INSERT INTO mosaic_tile
VALUES (16, 1, 5, NULL);
INSERT INTO mosaic_tile
VALUES (17, 1, 6, NULL);
INSERT INTO mosaic_tile
VALUES (18, 1, 7, NULL);
INSERT INTO mosaic_tile
VALUES (19, 1, 8, NULL);
INSERT INTO mosaic_tile
VALUES (20, 1, 9, NULL);
INSERT INTO mosaic_tile
VALUES (21, 2, 0, NULL);
INSERT INTO mosaic_tile
VALUES (22, 2, 1, NULL);
INSERT INTO mosaic_tile
VALUES (23, 2, 2, NULL);
INSERT INTO mosaic_tile
VALUES (24, 2, 3, NULL);
INSERT INTO mosaic_tile
VALUES (25, 2, 4, NULL);
INSERT INTO mosaic_tile
VALUES (26, 2, 5, NULL);
INSERT INTO mosaic_tile
VALUES (27, 2, 6, NULL);
INSERT INTO mosaic_tile
VALUES (28, 2, 7, NULL);
INSERT INTO mosaic_tile
VALUES (29, 2, 8, NULL);
INSERT INTO mosaic_tile
VALUES (30, 2, 9, NULL);
INSERT INTO mosaic_tile
VALUES (31, 3, 0, NULL);
INSERT INTO mosaic_tile
VALUES (32, 3, 1, NULL);
INSERT INTO mosaic_tile
VALUES (33, 3, 2, NULL);
INSERT INTO mosaic_tile
VALUES (34, 3, 3, NULL);
INSERT INTO mosaic_tile
VALUES (35, 3, 4, NULL);
INSERT INTO mosaic_tile
VALUES (36, 3, 5, NULL);
INSERT INTO mosaic_tile
VALUES (37, 3, 6, NULL);
INSERT INTO mosaic_tile
VALUES (38, 3, 7, NULL);
INSERT INTO mosaic_tile
VALUES (39, 3, 8, NULL);
INSERT INTO mosaic_tile
VALUES (40, 3, 9, NULL);
INSERT INTO mosaic_tile
VALUES (41, 4, 0, NULL);
INSERT INTO mosaic_tile
VALUES (42, 4, 1, NULL);
INSERT INTO mosaic_tile
VALUES (43, 4, 2, NULL);
INSERT INTO mosaic_tile
VALUES (44, 4, 3, NULL);
INSERT INTO mosaic_tile
VALUES (45, 4, 4, NULL);
INSERT INTO mosaic_tile
VALUES (46, 4, 5, NULL);
INSERT INTO mosaic_tile
VALUES (47, 4, 6, NULL);
INSERT INTO mosaic_tile
VALUES (48, 4, 7, NULL);
INSERT INTO mosaic_tile
VALUES (49, 4, 8, NULL);
INSERT INTO mosaic_tile
VALUES (50, 4, 9, NULL);
INSERT INTO mosaic_tile
VALUES (51, 5, 0, NULL);
INSERT INTO mosaic_tile
VALUES (52, 5, 1, NULL);
INSERT INTO mosaic_tile
VALUES (53, 5, 2, NULL);
INSERT INTO mosaic_tile
VALUES (54, 5, 3, NULL);
INSERT INTO mosaic_tile
VALUES (55, 5, 4, NULL);
INSERT INTO mosaic_tile
VALUES (56, 5, 5, NULL);
INSERT INTO mosaic_tile
VALUES (57, 5, 6, NULL);
INSERT INTO mosaic_tile
VALUES (58, 5, 7, NULL);
INSERT INTO mosaic_tile
VALUES (59, 5, 8, NULL);
INSERT INTO mosaic_tile
VALUES (60, 5, 9, NULL);
INSERT INTO mosaic_tile
VALUES (61, 6, 0, NULL);
INSERT INTO mosaic_tile
VALUES (62, 6, 1, NULL);
INSERT INTO mosaic_tile
VALUES (63, 6, 2, NULL);
INSERT INTO mosaic_tile
VALUES (64, 6, 3, NULL);
INSERT INTO mosaic_tile
VALUES (65, 6, 4, NULL);
INSERT INTO mosaic_tile
VALUES (66, 6, 5, NULL);
INSERT INTO mosaic_tile
VALUES (67, 6, 6, NULL);
INSERT INTO mosaic_tile
VALUES (68, 6, 7, NULL);
INSERT INTO mosaic_tile
VALUES (69, 6, 8, NULL);
INSERT INTO mosaic_tile
VALUES (70, 6, 9, NULL);
INSERT INTO mosaic_tile
VALUES (71, 7, 0, NULL);
INSERT INTO mosaic_tile
VALUES (72, 7, 1, NULL);
INSERT INTO mosaic_tile
VALUES (73, 7, 2, NULL);
INSERT INTO mosaic_tile
VALUES (74, 7, 3, NULL);
INSERT INTO mosaic_tile
VALUES (75, 7, 4, NULL);
INSERT INTO mosaic_tile
VALUES (76, 7, 5, NULL);
INSERT INTO mosaic_tile
VALUES (77, 7, 6, NULL);
INSERT INTO mosaic_tile
VALUES (78, 7, 7, NULL);
INSERT INTO mosaic_tile
VALUES (79, 7, 8, NULL);
INSERT INTO mosaic_tile
VALUES (80, 7, 9, NULL);
INSERT INTO mosaic_tile
VALUES (81, 8, 0, NULL);
INSERT INTO mosaic_tile
VALUES (82, 8, 1, NULL);
INSERT INTO mosaic_tile
VALUES (83, 8, 2, NULL);
INSERT INTO mosaic_tile
VALUES (84, 8, 3, NULL);
INSERT INTO mosaic_tile
VALUES (85, 8, 4, NULL);
INSERT INTO mosaic_tile
VALUES (86, 8, 5, NULL);
INSERT INTO mosaic_tile
VALUES (87, 8, 6, NULL);
INSERT INTO mosaic_tile
VALUES (88, 8, 7, NULL);
INSERT INTO mosaic_tile
VALUES (89, 8, 8, NULL);
INSERT INTO mosaic_tile
VALUES (90, 8, 9, NULL);
INSERT INTO mosaic_tile
VALUES (91, 9, 0, NULL);
INSERT INTO mosaic_tile
VALUES (92, 9, 1, NULL);
INSERT INTO mosaic_tile
VALUES (93, 9, 2, NULL);
INSERT INTO mosaic_tile
VALUES (94, 9, 3, NULL);
INSERT INTO mosaic_tile
VALUES (95, 9, 4, NULL);
INSERT INTO mosaic_tile
VALUES (96, 9, 5, NULL);
INSERT INTO mosaic_tile
VALUES (97, 9, 6, NULL);
INSERT INTO mosaic_tile
VALUES (98, 9, 7, NULL);
INSERT INTO mosaic_tile
VALUES (99, 9, 8, NULL);
INSERT INTO mosaic_tile
VALUES (100, 9, 9, NULL);
