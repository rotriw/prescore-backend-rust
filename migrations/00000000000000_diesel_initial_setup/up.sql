--
-- PostgreSQL database dump
--

-- Dumped from database version 14.9 (Ubuntu 14.9-1.pgdg20.04+1)
-- Dumped by pg_dump version 14.9 (Ubuntu 14.9-1.pgdg20.04+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', 'public', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: prescore; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA prescore;


ALTER SCHEMA prescore OWNER TO postgres;

--
-- Name: uuid-ossp; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA prescore;


--
-- Name: EXTENSION "uuid-ossp"; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';


--
-- Name: agg_class_info_from_examid(text); Type: FUNCTION; Schema: prescore; Owner: prescore
--

CREATE FUNCTION prescore.agg_class_info_from_examid(eid text) RETURNS TABLE(class_name text, class_id text, count bigint, max double precision, min double precision, avg double precision, med double precision)
    LANGUAGE plpgsql
    AS $$
BEGIN
    RETURN QUERY
    SELECT get_class_name(t.class_id) as class_name, t.class_id, count(e.user_score), max(e.user_score),
           min(e.user_score), avg(e.user_score),
           percentile_disc(0.5) within group (order by e.user_score) as med
    from (
        SELECT "exam".user_id, sum("exam".user_score) as user_score
        from "exam"
        where exam_id=eid
        group by "exam".user_id
        having count("exam".user_score)=(SELECT c.cnt from get_subject_count_from_examid(eid) c)
    ) e
    inner join (SELECT "user".child_id as child_id, max("user".class_id) as class_id FROM "user" GROUP BY "user".child_id) t
        on e.user_id = t.child_id
    group by t.class_id;
END
$$;


ALTER FUNCTION prescore.agg_class_info_from_examid(eid text) OWNER TO prescore;

--
-- Name: agg_class_info_from_paperid(text); Type: FUNCTION; Schema: prescore; Owner: prescore
--

CREATE FUNCTION prescore.agg_class_info_from_paperid(pid text) RETURNS TABLE(class_name text, class_id text, count bigint, max double precision, min double precision, avg double precision, med double precision)
    LANGUAGE plpgsql
    AS $$
BEGIN
    RETURN QUERY
    SELECT get_class_name(t.class_id) as class_name, t.class_id, count("exam".user_score), max("exam".user_score),
           min("exam".user_score), avg("exam".user_score),
           percentile_disc(0.5) within group (order by "exam".user_score) as med
    from "exam"
    inner join (SELECT "user".child_id as child_id, max("user".class_id) as class_id FROM "user" GROUP BY "user".child_id) t
        on exam.user_id = t.child_id
    where paper_id=pid
    group by t.class_id;
END;
$$;


ALTER FUNCTION prescore.agg_class_info_from_paperid(pid text) OWNER TO prescore;

--
-- Name: agg_exam_info_from_classid(text, text); Type: FUNCTION; Schema: prescore; Owner: prescore
--

CREATE FUNCTION prescore.agg_exam_info_from_classid(cid text, eid text) RETURNS TABLE(count bigint, max double precision, min double precision, avg double precision, med double precision)
    LANGUAGE plpgsql
    AS $$
BEGIN
    RETURN QUERY
    SELECT count(s_sum), max(s_sum), min(s_sum), avg(s_sum), percentile_disc(0.5) within group (order by s_sum)
        from (
        SELECT sum("exam".user_score) as s_sum
        from "exam"
        where "exam".exam_id=eid and exam.user_id in (SELECT r.user_id FROM get_users_from_class(cid) r)
        group by "exam".user_id
        having count(*)=(SELECT count(distinct "exam".paper_id) FROM "exam" where "exam".exam_id=eid)
    ) as ess;
END;
$$;


ALTER FUNCTION prescore.agg_exam_info_from_classid(cid text, eid text) OWNER TO prescore;

--
-- Name: agg_paper_info_from_classid(text, text); Type: FUNCTION; Schema: prescore; Owner: prescore
--

CREATE FUNCTION prescore.agg_paper_info_from_classid(cid text, pid text) RETURNS TABLE(count bigint, max double precision, min double precision, avg double precision, med double precision)
    LANGUAGE plpgsql
    AS $$
BEGIN
    RETURN QUERY
    SELECT count(user_score), max(user_score), min(user_score), avg(user_score), percentile_disc(0.5) within group (order by user_score)
    from "exam"
    where paper_id=pid and exam.user_id in (SELECT t.user_id FROM get_users_from_class(cid) t);
END;
$$;


ALTER FUNCTION prescore.agg_paper_info_from_classid(cid text, pid text) OWNER TO prescore;

--
-- Name: get_class_name(text); Type: FUNCTION; Schema: prescore; Owner: prescore
--

CREATE FUNCTION prescore.get_class_name(cid text) RETURNS text
    LANGUAGE plpgsql
    AS $$
DECLARE
    val text;
BEGIN
    SELECT string_agg(a.class_name, ' / ') INTO val from (SELECT distinct "user".class_name FROM "user" WHERE class_id=cid AND class_name!='' ORDER BY class_name) a WHERE a.class_name != '';
    IF val IS NULL THEN
        val := '';
    END IF;
    RETURN val;
END
$$;


ALTER FUNCTION prescore.get_class_name(cid text) OWNER TO prescore;

--
-- Name: get_classes_from_examid(text); Type: FUNCTION; Schema: prescore; Owner: prescore
--

CREATE FUNCTION prescore.get_classes_from_examid(eid text) RETURNS TABLE(class_name text, class_id text)
    LANGUAGE plpgsql
    AS $$
BEGIN
    CREATE TEMP TABLE classes
    (
        class_name text,
        class_id text
    ) ON COMMIT DROP;

    INSERT INTO classes
    SELECT DISTINCT get_class_name("user".class_id), "user".class_id
    FROM (
        SELECT DISTINCT user_id FROM "exam" where exam_id=eid
    ) AS ex,
    "user"
    WHERE "user".user_id=ex.user_id
    ORDER BY class_id;

    RETURN QUERY
    SELECT t.class_name, t.class_id FROM classes t;
END;
$$;


ALTER FUNCTION prescore.get_classes_from_examid(eid text) OWNER TO prescore;

--
-- Name: get_classes_from_paperid(text); Type: FUNCTION; Schema: prescore; Owner: prescore
--

CREATE FUNCTION prescore.get_classes_from_paperid(pid text) RETURNS TABLE(class_name text, class_id text)
    LANGUAGE plpgsql
    AS $$
BEGIN
    CREATE TEMP TABLE classes
    (
        class_name text,
        class_id text
    ) ON COMMIT DROP;

    INSERT INTO classes
    SELECT DISTINCT get_class_name("user".class_id), "user".class_id
        FROM (
            SELECT DISTINCT user_id FROM "exam" where paper_id=pid
        ) AS ex,
        "user"
        WHERE "user".user_id=ex.user_id
        ORDER BY class_id;

    RETURN QUERY
    SELECT t.class_name, t.class_id FROM classes t;
END;
$$;


ALTER FUNCTION prescore.get_classes_from_paperid(pid text) OWNER TO prescore;

--
-- Name: get_percentage_from_paperid(text, double precision); Type: FUNCTION; Schema: prescore; Owner: prescore
--

CREATE FUNCTION prescore.get_percentage_from_paperid(pid text, usc double precision) RETURNS TABLE(pct double precision)
    LANGUAGE plpgsql
    AS $$
BEGIN
    RETURN QUERY
    SELECT (count(ret.class_id) * 100 - sum(ret.var_diag)) / (count(ret.class_id) * 100) as pct
    from (
        SELECT t.class_id, e.user_score,
               (CASE WHEN
                   (CASE WHEN (usc-va.maximum > 10)
                       THEN 100 ELSE e.diag_score + (usc - e.user_score) * variation * 2 END) > 100
                   THEN 100 ELSE (
                       CASE WHEN
                        (CASE WHEN usc-va.maximum > 10
                            THEN 100 ELSE e.diag_score + (usc - e.user_score) * variation * 2 END) < 0
                        THEN 0 ELSE (CASE WHEN usc-va.maximum > 10
                            THEN 100 ELSE e.diag_score + (usc - e.user_score) * variation * 2 END) END
                   )
                   END) as var_diag, row_number() over (partition by t.class_id order by (abs(usc - e.user_score))) as row_num
        from (
            SELECT "exam".user_id, "exam".user_score as user_score, "exam".subject_name, "exam".subject_id, "exam".diagnostic_score as diag_score
            from "exam"
            where paper_id=pid and diagnostic_score is not null
        ) e
        inner join (SELECT "user".child_id as child_id, max("user".class_id) as class_id FROM "user" GROUP BY "user".child_id) t
            on e.user_id = t.child_id
        inner join (
            SELECT t.class_id, max(e.user_score) as maximum,(max(e.user_score) - min(e.user_score)) / (max(e.diag_score) - min(e.diag_score)) as variation
            from (
                SELECT "exam".user_id, "exam".user_score as user_score, "exam".subject_name, "exam".subject_id, "exam".diagnostic_score as diag_score
                from "exam"
                where paper_id=pid and diagnostic_score is not null
            ) e
            inner join (SELECT "user".child_id as child_id, max("user".class_id) as class_id FROM "user" GROUP BY "user".child_id) t
                on e.user_id = t.child_id
            group by t.class_id
            having max(diag_score) - min(diag_score) != 0
            order by class_id
        ) va
            on t.class_id = va.class_id
        order by row_num, class_id
    ) ret
    where ret.row_num=1;
END
$$;


ALTER FUNCTION prescore.get_percentage_from_paperid(pid text, usc double precision) OWNER TO prescore;

--
-- Name: get_subject_count_from_examid(text); Type: FUNCTION; Schema: prescore; Owner: prescore
--

CREATE FUNCTION prescore.get_subject_count_from_examid(eid text) RETURNS TABLE(cnt bigint)
    LANGUAGE plpgsql
    AS $$
BEGIN
    RETURN QUERY
    SELECT max(t.cnt) as cnt FROM (SELECT count(1) as cnt FROM exam WHERE exam_id=eid GROUP BY user_id) t;
END
$$;


ALTER FUNCTION prescore.get_subject_count_from_examid(eid text) OWNER TO prescore;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: user; Type: TABLE; Schema: prescore; Owner: prescore
--

CREATE TABLE prescore."user" (
    id bigint NOT NULL,
    user_id text,
    name text,
    login_name text,
    hashed_token text,
    school_id text,
    school_name text,
    division_id text,
    division_name text,
    class_id text,
    class_name text,
    child_id text
);


ALTER TABLE prescore."user" OWNER TO prescore;

--
-- Name: get_users_from_class(text); Type: FUNCTION; Schema: prescore; Owner: prescore
--

CREATE FUNCTION prescore.get_users_from_class(cid text) RETURNS SETOF prescore."user"
    LANGUAGE plpgsql
    AS $$
BEGIN
    RETURN QUERY
    SELECT * FROM "user" WHERE class_id=cid;
END;
$$;


ALTER FUNCTION prescore.get_users_from_class(cid text) OWNER TO prescore;

--
-- Name: exam; Type: TABLE; Schema: prescore; Owner: prescore
--

CREATE TABLE prescore.exam (
    id bigint NOT NULL,
    user_id text NOT NULL,
    exam_id text NOT NULL,
    paper_id text NOT NULL,
    subject_name text,
    subject_id text,
    standard_score double precision,
    user_score double precision,
    diagnostic_score double precision
)
WITH (parallel_workers='16');


ALTER TABLE prescore.exam OWNER TO prescore;

--
-- Name: exam_id_seq; Type: SEQUENCE; Schema: prescore; Owner: prescore
--

CREATE SEQUENCE prescore.exam_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE prescore.exam_id_seq OWNER TO prescore;

--
-- Name: exam_id_seq; Type: SEQUENCE OWNED BY; Schema: prescore; Owner: prescore
--

ALTER SEQUENCE prescore.exam_id_seq OWNED BY prescore.exam.id;


--
-- Name: user_id_seq; Type: SEQUENCE; Schema: prescore; Owner: prescore
--

CREATE SEQUENCE prescore.user_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE prescore.user_id_seq OWNER TO prescore;

--
-- Name: user_id_seq; Type: SEQUENCE OWNED BY; Schema: prescore; Owner: prescore
--

ALTER SEQUENCE prescore.user_id_seq OWNED BY prescore."user".id;


--
-- Name: exam id; Type: DEFAULT; Schema: prescore; Owner: prescore
--

ALTER TABLE ONLY prescore.exam ALTER COLUMN id SET DEFAULT nextval('prescore.exam_id_seq'::regclass);


--
-- Name: user id; Type: DEFAULT; Schema: prescore; Owner: prescore
--

ALTER TABLE ONLY prescore."user" ALTER COLUMN id SET DEFAULT nextval('prescore.user_id_seq'::regclass);


--
-- Data for Name: exam; Type: TABLE DATA; Schema: prescore; Owner: prescore
--


--
-- Name: exam_id_seq; Type: SEQUENCE SET; Schema: prescore; Owner: prescore
--

SELECT pg_catalog.setval('prescore.exam_id_seq', 100209, true);


--
-- Name: user_id_seq; Type: SEQUENCE SET; Schema: prescore; Owner: prescore
--

SELECT pg_catalog.setval('prescore.user_id_seq', 2056, true);


--
-- Name: exam idx_16460_primary; Type: CONSTRAINT; Schema: prescore; Owner: prescore
--

ALTER TABLE ONLY prescore.exam
    ADD CONSTRAINT idx_16460_primary PRIMARY KEY (id);


--
-- Name: user idx_16465_primary; Type: CONSTRAINT; Schema: prescore; Owner: prescore
--

ALTER TABLE ONLY prescore."user"
    ADD CONSTRAINT idx_16465_primary PRIMARY KEY (id);


--
-- Name: index_exam_paper_id; Type: INDEX; Schema: prescore; Owner: prescore
--

CREATE INDEX index_exam_paper_id ON prescore.exam USING btree (exam_id, paper_id);


--
-- Name: index_name; Type: INDEX; Schema: prescore; Owner: prescore
--

CREATE UNIQUE INDEX index_name ON prescore.exam USING btree (user_id, exam_id, paper_id);


--
-- Name: index_paper_id; Type: INDEX; Schema: prescore; Owner: prescore
--

CREATE INDEX index_paper_id ON prescore.exam USING btree (paper_id);


--
-- Name: index_uid; Type: INDEX; Schema: prescore; Owner: prescore
--

CREATE INDEX index_uid ON prescore.exam USING btree (user_id);


--
-- Name: index_user_class; Type: INDEX; Schema: prescore; Owner: prescore
--

CREATE INDEX index_user_class ON prescore."user" USING btree (user_id, class_id, class_name);


--
-- Name: SCHEMA prescore; Type: ACL; Schema: -; Owner: postgres
--

GRANT ALL ON SCHEMA prescore TO prescore;


--
-- PostgreSQL database dump complete
--

