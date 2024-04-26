CREATE SEQUENCE prescore.test_number_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TABLE "prescore"."test_number" (
    "id" int8 NOT NULL DEFAULT nextval('test_number_seq'::regclass),
    "paper_id" text NOT NULL,
    "class_id" text NOT NULL,
    "number" int8 NOT NULL,
    PRIMARY KEY ("id")
);

ADD CONSTRAINT "unique" UNIQUE ("paper_id", "class_id");