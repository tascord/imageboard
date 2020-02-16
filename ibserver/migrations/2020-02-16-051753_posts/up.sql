CREATE TABLE "posts"
(
 "post_id"       serial NOT NULL,
 "creation_date" date NOT NULL,
 "hash"          char(32) NOT NULL,
 "original_name" varchar(255) NOT NULL,
 "height"        int NOT NULL,
 "width"         int NOT NULL,
 "ext"           varchar(5) NOT NULL,
 "score"         int NOT NULL,
 "directory"     char(3) NOT NULL,
 "ip"            varchar(39) NOT NULL,
 CONSTRAINT "PK_posts" PRIMARY KEY ( "post_id" )
);