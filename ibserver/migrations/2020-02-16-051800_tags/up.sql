CREATE TABLE "tags"
(
 "tag_id" serial NOT NULL,
 "name"   varchar(255) NOT NULL,
 "count"  int NOT NULL,
 CONSTRAINT "PK_tags" PRIMARY KEY ( "tag_id" )
);