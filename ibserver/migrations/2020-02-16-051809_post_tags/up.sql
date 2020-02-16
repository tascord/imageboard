CREATE TABLE "post_tags"
(
 "id"      serial NOT NULL,
 "tag_id"  serial NOT NULL,
 "post_id" serial NOT NULL,
 CONSTRAINT "PK_post_tags" PRIMARY KEY ( "id" ),
 CONSTRAINT "posts-post_tags" FOREIGN KEY ( "post_id" ) REFERENCES "posts" ( "post_id" ),
 CONSTRAINT "tags-post_tags" FOREIGN KEY ( "tag_id" ) REFERENCES "tags" ( "tag_id" )
);

CREATE INDEX "fkIdx_0" ON "post_tags"
(
 "post_id"
);

CREATE INDEX "fkIdx_1" ON "post_tags"
(
 "tag_id"
);