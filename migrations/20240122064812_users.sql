-- Add migration script here
CREATE TABLE IF NOT EXISTS via_users(
   id serial PRIMARY KEY NOT NULL,
   name varchar(255) NOT NULL,
   password varchar(255) NOT NULL,
   role_id int NOT NULL,
   deleted boolean NOT NULL DEFAULT false,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
  );

  INSERT INTO "public"."via_users" ("name", "password", "role_id", "deleted", "created_at", "updated_at") 
  VALUES ('gnss', 'gnss@123', 1, 'f', '2024-01-22 15:01:10', '2024-01-22 15:01:13');