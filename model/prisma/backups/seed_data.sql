--
-- PostgreSQL database dump
--

-- Dumped from database version 15.10 (Debian 15.10-0+deb12u1)
-- Dumped by pg_dump version 15.10 (Debian 15.10-0+deb12u1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;


--
-- Name: InvoiceStatus; Type: TYPE; Schema: public; Owner: pat
--

CREATE TYPE public."InvoiceStatus" AS ENUM (
    'PENDING',
    'PAID',
    'OVERDUE',
    'DRAFT'
);


ALTER TYPE public."InvoiceStatus" OWNER TO pat;

SET default_tablespace = '';

SET default_table_access_method = heap;

DROP TABLE public."Revenue";
DROP TABLE public."Invoice";
DROP TABLE public."Customer";
DROP TABLE public."User";

--
-- Name: Customer; Type: TABLE; Schema: public; Owner: pat
--

CREATE TABLE public."Customer" (
    id text NOT NULL,
    name text NOT NULL,
    email text NOT NULL,
    "imageUrl" text,
    "createdAt" timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updatedAt" timestamp(3) without time zone NOT NULL
);


ALTER TABLE public."Customer" OWNER TO pat;

--
-- Name: Invoice; Type: TABLE; Schema: public; Owner: pat
--

CREATE TABLE public."Invoice" (
    id text NOT NULL,
    "customerId" text NOT NULL,
    amount integer NOT NULL,
    date timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "createdAt" timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updatedAt" timestamp(3) without time zone NOT NULL,
    status public."InvoiceStatus" DEFAULT 'PENDING'::public."InvoiceStatus" NOT NULL
);


ALTER TABLE public."Invoice" OWNER TO pat;

--
-- Name: Revenue; Type: TABLE; Schema: public; Owner: pat
--

CREATE TABLE public."Revenue" (
    id text NOT NULL,
    month timestamp(3) without time zone NOT NULL,
    revenue integer NOT NULL,
    expenses integer NOT NULL
);


ALTER TABLE public."Revenue" OWNER TO pat;

--
-- Name: User; Type: TABLE; Schema: public; Owner: pat
--

CREATE TABLE public."User" (
    id text NOT NULL,
    name text NOT NULL,
    email text NOT NULL,
    password text NOT NULL,
    "createdAt" timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updatedAt" timestamp(3) without time zone NOT NULL
);


ALTER TABLE public."User" OWNER TO pat;

--
-- Name: _prisma_migrations; Type: TABLE; Schema: public; Owner: pat
--

CREATE TABLE public._prisma_migrations (
    id character varying(36) NOT NULL,
    checksum character varying(64) NOT NULL,
    finished_at timestamp with time zone,
    migration_name character varying(255) NOT NULL,
    logs text,
    rolled_back_at timestamp with time zone,
    started_at timestamp with time zone DEFAULT now() NOT NULL,
    applied_steps_count integer DEFAULT 0 NOT NULL
);


ALTER TABLE public._prisma_migrations OWNER TO pat;

--
-- Data for Name: Customer; Type: TABLE DATA; Schema: public; Owner: pat
--

COPY public."Customer" (id, name, email, "imageUrl", "createdAt", "updatedAt") FROM stdin;
868c2ec0-4d81-4a14-af49-bde33af4dc95	Evil Rabbit	evil@rabbit.com	/customers/evil-rabbit.png	2025-02-26 02:12:46.692	2025-02-26 02:12:46.692
a4f6cd26-04a0-499e-856a-cb6eee0930eb	Delba de Oliveira	delba@oliveira.com	/customers/delba-de-oliveira.png	2025-02-26 02:12:46.697	2025-02-26 02:12:46.697
41f56d9b-bf8d-4a4e-8cc1-9a5fa86e99e6	Lee Robinson	lee@robinson.com	/customers/lee-robinson.png	2025-02-26 02:12:46.701	2025-02-26 02:12:46.701
c322608a-698b-45f1-a876-d8fa422ed85d	Michael Novotny	michael@novotny.com	/customers/michael-novotny.png	2025-02-26 02:12:46.708	2025-02-26 02:12:46.708
603da5a0-8b45-450d-b471-37e8d5299517	Amy Burns	amy@burns.com	/customers/amy-burns.png	2025-02-26 02:12:46.71	2025-02-26 02:12:46.71
c232705a-7174-4fce-af36-dcfc2170c574	Balazs Orban	balazs@orban.com	/customers/balazs-orban.png	2025-02-26 02:12:46.712	2025-02-26 02:12:46.712
\.


--
-- Data for Name: Invoice; Type: TABLE DATA; Schema: public; Owner: pat
--

COPY public."Invoice" (id, "customerId", amount, date, "createdAt", "updatedAt", status) FROM stdin;
ae1e0847-1667-4967-bac5-b165ccad476f	868c2ec0-4d81-4a14-af49-bde33af4dc95	15795	2022-12-06 00:00:00	2025-02-26 02:12:46.716	2025-02-26 02:12:46.716	PENDING
64aad11a-3de6-476d-89e4-238662e97a7c	a4f6cd26-04a0-499e-856a-cb6eee0930eb	20348	2022-11-14 00:00:00	2025-02-26 02:12:46.727	2025-02-26 02:12:46.727	PENDING
3243e525-56cd-4520-9453-4871018f932b	603da5a0-8b45-450d-b471-37e8d5299517	3040	2022-10-29 00:00:00	2025-02-26 02:12:46.737	2025-02-26 02:12:46.737	PAID
15ad68e0-5b9d-4cad-87e6-82f3afd02b1f	c322608a-698b-45f1-a876-d8fa422ed85d	44800	2023-09-10 00:00:00	2025-02-26 02:12:46.74	2025-02-26 02:12:46.74	PAID
57058e5a-f850-4847-bf3f-fed59157e8eb	c232705a-7174-4fce-af36-dcfc2170c574	34577	2023-08-05 00:00:00	2025-02-26 02:12:46.745	2025-02-26 02:12:46.745	PENDING
82ca881f-04d4-4072-a0d3-84ebd6198479	41f56d9b-bf8d-4a4e-8cc1-9a5fa86e99e6	54246	2023-07-16 00:00:00	2025-02-26 02:12:46.748	2025-02-26 02:12:46.748	PENDING
a431c98c-63f1-4777-a9d9-e9be261321d7	868c2ec0-4d81-4a14-af49-bde33af4dc95	666	2023-06-27 00:00:00	2025-02-26 02:12:46.751	2025-02-26 02:12:46.751	PENDING
fa00c093-88f2-431c-8b62-57ef0a95472c	c322608a-698b-45f1-a876-d8fa422ed85d	32545	2023-06-09 00:00:00	2025-02-26 02:12:46.762	2025-02-26 02:12:46.762	PAID
6d2a0dc2-518d-4401-9a45-d507071500ae	603da5a0-8b45-450d-b471-37e8d5299517	1250	2023-06-17 00:00:00	2025-02-26 02:12:46.765	2025-02-26 02:12:46.765	PAID
fdf34947-9979-4541-8073-2dfc2139513a	c232705a-7174-4fce-af36-dcfc2170c574	8546	2023-06-07 00:00:00	2025-02-26 02:12:46.768	2025-02-26 02:12:46.768	PAID
cf0606af-0082-48f4-9026-d08594265627	a4f6cd26-04a0-499e-856a-cb6eee0930eb	500	2023-08-19 00:00:00	2025-02-26 02:12:46.771	2025-02-26 02:12:46.771	PAID
fcbcff2f-9c9f-490a-8d70-1467fe1dcef2	c232705a-7174-4fce-af36-dcfc2170c574	8945	2023-06-03 00:00:00	2025-02-26 02:12:46.774	2025-02-26 02:12:46.774	PAID
b5ed1abd-9520-4eda-8ed5-6a5206c90e63	41f56d9b-bf8d-4a4e-8cc1-9a5fa86e99e6	1000	2022-06-05 00:00:00	2025-02-26 02:12:46.776	2025-02-26 02:12:46.776	PAID
\.


--
-- Data for Name: Revenue; Type: TABLE DATA; Schema: public; Owner: pat
--

COPY public."Revenue" (id, month, revenue, expenses) FROM stdin;
2bbb215b-0707-4e87-a3b7-197c96da1070	2025-01-01 08:00:00	2000	0
0a4d1478-4354-4608-b34c-e374e0e84271	2025-02-01 08:00:00	1800	0
3e2065a1-0c7e-44e7-b19a-a0d8da5a2cd0	2025-03-01 08:00:00	2200	0
947aae00-de51-4896-b293-69138c8d89bf	2025-04-01 07:00:00	2500	0
970d838c-a360-4a14-b27d-e8ebd422722b	2025-05-01 07:00:00	2300	0
0fc1bdfe-d70c-4a42-b21b-9b4b6059533a	2025-06-01 07:00:00	3200	0
8dd0c048-e1c4-423a-8e8a-5ee0b4742675	2025-07-01 07:00:00	3500	0
2180571c-923f-4a7e-b68f-4b8bab6aa42e	2025-08-01 07:00:00	3700	0
48209b30-7847-4a4c-8ced-8d6906f1d247	2025-09-01 07:00:00	2500	0
43b242a4-c518-4d63-b9f2-9e1fcb3f09a2	2025-10-01 07:00:00	2800	0
d9951b8d-ccad-45bf-9730-55ae1ba5a5b6	2025-11-01 07:00:00	3000	0
a5e962b7-784a-43d1-8b0b-06000249e075	2025-12-01 08:00:00	4800	0
\.


--
-- Data for Name: User; Type: TABLE DATA; Schema: public; Owner: pat
--

COPY public."User" (id, name, email, password, "createdAt", "updatedAt") FROM stdin;
\.

--
-- Name: Customer Customer_pkey; Type: CONSTRAINT; Schema: public; Owner: pat
--

ALTER TABLE ONLY public."Customer"
    ADD CONSTRAINT "Customer_pkey" PRIMARY KEY (id);


--
-- Name: Invoice Invoice_pkey; Type: CONSTRAINT; Schema: public; Owner: pat
--

ALTER TABLE ONLY public."Invoice"
    ADD CONSTRAINT "Invoice_pkey" PRIMARY KEY (id);


--
-- Name: Revenue Revenue_pkey; Type: CONSTRAINT; Schema: public; Owner: pat
--

ALTER TABLE ONLY public."Revenue"
    ADD CONSTRAINT "Revenue_pkey" PRIMARY KEY (id);


--
-- Name: User User_pkey; Type: CONSTRAINT; Schema: public; Owner: pat
--

ALTER TABLE ONLY public."User"
    ADD CONSTRAINT "User_pkey" PRIMARY KEY (id);


--
-- Name: _prisma_migrations _prisma_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: pat
--

ALTER TABLE ONLY public._prisma_migrations
    ADD CONSTRAINT _prisma_migrations_pkey PRIMARY KEY (id);


--
-- Name: Customer_email_key; Type: INDEX; Schema: public; Owner: pat
--

CREATE UNIQUE INDEX "Customer_email_key" ON public."Customer" USING btree (email);


--
-- Name: User_email_key; Type: INDEX; Schema: public; Owner: pat
--

CREATE UNIQUE INDEX "User_email_key" ON public."User" USING btree (email);


--
-- Name: Invoice Invoice_customerId_fkey; Type: FK CONSTRAINT; Schema: public; Owner: pat
--

ALTER TABLE ONLY public."Invoice"
    ADD CONSTRAINT "Invoice_customerId_fkey" FOREIGN KEY ("customerId") REFERENCES public."Customer"(id) ON UPDATE CASCADE ON DELETE RESTRICT;


--
-- PostgreSQL database dump complete
--

