INSERT INTO user_info (id, user_type, firstname, middlename, lastname, cid, ctime, mid, mtime) VALUES (0, 'super', 'john', 'doe', 'sur', 0 now(), 0, now());

INSERT INTO user_contact_info (id, user_id, email_address, contact_number, cid, ctime, mid, mtime) VALUES (0, 0, 'john@email.com', 0976543212, 0, now(), 0, now());

INSERT INTO user_job (id, user_id, occupation, position, cid, ctime, mid, mtime) VALUES (0, 0, 'web dev', 'manager', 0, now(), 0, now());

INSERT INTO user_picture (id, user_id, img_path, cid, ctime, mid, mtime) VALUES (0, 0, 'https://www.pexels.com/photo/portrait-of-a-man-sitting-9243888/', 0, now(), 0, now());

INSERT INTO user_login (id, user_id, username, password, pass_salt, token_salt, cid, ctime, mid, mtime) VALUES (0, 0, 'john123', );
