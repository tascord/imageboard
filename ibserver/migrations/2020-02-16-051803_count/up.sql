CREATE TABLE "count"
(
 "hit_count"  int NOT NULL,
 "id"         serial NOT NULL,
 "post_count" int NOT NULL,
 CONSTRAINT "PK_count" PRIMARY KEY ( "id" )
);