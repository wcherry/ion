INSERT INTO users (id,name,email_address,profile_id,password,role) VALUES (1,'admin', 'admin@ion.con', 1, '$argon2id$v=19$m=19456,t=2,p=1$BNMFKQdaJTks/xvKH3YDeA$nC/vRxLWR4ofv2cI6H3JGzFxGStV6lBrm+KOwmMn4ak','admin')
;
INSERT INTO profile (id, default_page_id) VALUES (1, uuid_generate_v4())
;
insert into pages (id, name, owner_id) values ('66dd25a9-01ca-47ee-a558-31346e25ab8d', 'Admin Page', 1)
;
insert into page_versions (id, page_id, version) values (uuid_generate_v4(), '66dd25a9-01ca-47ee-a558-31346e25ab8d', 1)
;
insert into blocks (id, block_id, version, block_type, content) values('ea636765-dae1-495e-bda5-a55d74284449', uuid_generate_v4(), 1, 'Paragraph', 'A block of text to allow for testing')
;
insert into page_block_index (id, page_version_id, display_order, block_id) values (uuid_generate_v4(),'ea636765-dae1-495e-bda5-a55d74284449' ,2,'bcbae3f9-0a2d-4121-b93d-92362c320ff0')
;

