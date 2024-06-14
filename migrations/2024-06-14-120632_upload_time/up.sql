-- Your SQL goes here
CREATE SEQUENCE prescore.times_number_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER TABLE prescore.times_number_seq OWNER TO prescore;


CREATE TABLE "prescore"."times_number" (
    "id" int8 NOT NULL,
    "tid" int8 NOT NULL,
    "time" int8 NOT NULL,
    PRIMARY KEY ("id")
);

ALTER TABLE prescore.times_number OWNER TO prescore;

ALTER SEQUENCE prescore.times_number_seq OWNED BY prescore."times_number".id;

ALTER TABLE ONLY prescore.times_number ALTER COLUMN id SET DEFAULT nextval('prescore.times_number_seq'::regclass);

ADD CONSTRAINT "unique" UNIQUE ("id", "tid");