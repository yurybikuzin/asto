-- https://dba.stackexchange.com/questions/122742/how-to-drop-all-of-my-functions-in-postgresql
DO
$do$
DECLARE
   _sql text;
BEGIN
   SELECT INTO _sql
          string_agg(format('DROP %s %s;'
                          , CASE prokind
                              WHEN 'f' THEN 'FUNCTION'
                              WHEN 'a' THEN 'AGGREGATE'
                              WHEN 'p' THEN 'PROCEDURE'
                              WHEN 'w' THEN 'FUNCTION'  -- window function (rarely applicable)
                              -- ELSE NULL              -- not possible in pg 11
                            END
                          , oid::regprocedure)
                   , E'\n')
   FROM   pg_proc
   WHERE  pronamespace = 'public'::regnamespace  -- schema name here!
   -- AND    prokind = ANY ('{f,a,p,w}')         -- optionally filter kinds
   ;

   IF _sql IS NOT NULL THEN
      RAISE NOTICE '%', _sql;  -- debug / check first
       EXECUTE _sql;         -- uncomment payload once you are sure
   ELSE 
      RAISE NOTICE 'No fuction found in schema %', quote_ident(_schema);
   END IF;

   -- SELECT INTO _sql
   --        string_agg(format('DROP table if exists "%s" cascade;'
   --                        , tablename)
   --                 , E'\n')
   -- FROM   pg_tables
   -- WHERE  schemaname = 'public'  -- schema name here!
   -- -- AND    prokind = ANY ('{f,a,p,w}')         -- optionally filter kinds
   -- ;
   --
   -- IF _sql IS NOT NULL THEN
   --    RAISE NOTICE '%', _sql;  -- debug / check first
   --     EXECUTE _sql;         -- uncomment payload once you are sure
   -- ELSE 
   --    RAISE NOTICE 'No table found in schema %', quote_ident(_schema);
   -- END IF;

END
$do$;

-- select add_dancer( -1::smallint
-- 	, null::int
-- 	, add_person(-1::smallint
-- 		, null::int
-- 		, 'Мотуз'::text
-- 		, 'Михаил'::text
-- 		, 'Денисович'::text
-- 		, ''::text
-- 		, '2009-03-06'::date
-- 	)::int
-- 	, 5500194::int
-- 	, add_club(-1::smallint, null::int, 'Нева', 'С.-Петербург')::int
-- 	, add_trainer(-1::smallint, null::int, 'Андрейченко', 'Андрей', 'Нева', 'С.-Петербург')
-- 	, (select id from classlar where value = 'D')
-- 	, (select id from classlar where value = 'D')
-- 	, 0::int
-- 	, 0::int
-- 	, 25::int
-- 	, 0::int
-- );
--
-- select st_la_score from dancerlar(0::smallint, null) where external_id = 5500194;

-- create or replace function add_dancer(_op_mode smallint
--     , _id integer
--     , _person integer
--     , _external_id integer
--     , _club integer
--     , _trainer integer
--     , _st_class smallint
--     , _la_class smallint
--     , _st_score integer
--     , _la_score integer
--     , _st_la_score integer
--     , _points integer
-- )
--  RETURNS integer
--  LANGUAGE plpgsql
-- AS $$
-- declare 
-- 	_ret int;
-- 	_need_add bool = false;
-- begin
-- 	_external_id = coalesce(_external_id, 0);
-- 	_st_score = coalesce(_st_score, 0); 
-- 	_la_score = coalesce(_la_score, 0); 
-- 	_st_la_score = coalesce(_st_la_score, 0); 
-- 	_points = coalesce(_points, 0); 
--
-- 	select 
-- 		into _id
-- 		id
-- 	from dancerlar(_op_mode, null) 
-- 	where (_id is null or id = _id)
-- 		and (
-- 			_external_id != 0 and external_id = _external_id 
-- 			or 
-- 			_external_id = 0 and person = _person
-- 		)
-- 	limit 1;
-- 	if _id is null then
-- 		_need_add = true;
-- 	else 
-- 		if not exists (
-- 			select id
-- 			from dancerlar(_op_mode, null) 
-- 			where id = _id
-- 				and person = _person 
-- 				and external_id = _external_id 
-- 				and club = _club
-- 				and trainer = _trainer
-- 				and st_class = _st_class 
-- 				and la_class = _la_class 
-- 				and st_score = _st_score 
-- 				and la_score = _la_score 
-- 				and st_la_score = _st_la_score 
-- 				and points = _points 
-- 			limit 1
-- 		) then
-- 			_need_add = true;
-- 		end if;
-- 	end if;
--
-- 	if not _need_add then
-- 		_ret = _id;
-- 	else
-- 		insert into dancerlar (op_mode, id 
-- 			, person 
-- 			, external_id
-- 			, club
-- 			, trainer
-- 			, st_class
-- 			, la_class
-- 			, st_score
-- 			, la_score
-- 			, st_la_score
-- 			, points
-- 		) 
-- 		values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from dancerlar))
-- 			, _person 
-- 			, _external_id
-- 			, _club
-- 			, _trainer
-- 			, _st_class
-- 			, _la_class
-- 			, _st_score
-- 			, _la_score
-- 			, _st_la_score
-- 			, _points
-- 		)
-- 		on conflict (id, created_at, op_mode)
-- 		do update set op_mode = excluded.op_mode
-- 			, person = excluded.person 
-- 			, external_id = excluded.external_id
-- 			, club = excluded.club
-- 			, trainer = excluded.trainer
-- 			, st_class = excluded.st_class 
-- 			, la_class = excluded.la_class 
-- 			, st_score = excluded.st_score 
-- 			, la_score = excluded.la_score 
-- 			, st_la_score = excluded.st_la_score 
-- 			, points = excluded.points 
-- 		returning id into _ret;
-- 	end if;
--
-- 	return _ret;
-- end;
-- $$
-- ;

create or replace function add_dancer(_op_mode smallint
    , _id integer
    , _person integer
    , _external_id integer
    , _club integer
    , _trainer integer
    , _trainer2 integer
    , _st_class smallint
    , _la_class smallint
    , _st_score integer
    , _la_score integer
    , _st_la_score integer
    , _points integer
    , _is_archive bool
)
 RETURNS integer
 LANGUAGE plpgsql
AS $$
declare 
	_ret int;
	_need_add bool = false;
begin
	_external_id = coalesce(_external_id, 0);
	_st_score = coalesce(_st_score, 0); 
	_la_score = coalesce(_la_score, 0); 
	_st_la_score = coalesce(_st_la_score, 0); 
	_points = coalesce(_points, 0); 
    _is_archive = coalesce(_is_archive, false);

	select 
		into _id
		id
	from dancerlar(_op_mode, null) 
	where (_id is null or id = _id)
		and (
			_external_id != 0 and external_id = _external_id 
			or 
			_external_id = 0 and person = _person
		)
	limit 1;
	if _id is null then
		_need_add = true;
	else 
		if not exists (
			select id
			from dancerlar(_op_mode, null) 
			where id = _id
				and person = _person 
				and external_id = _external_id 
				and club = _club
				and trainer = _trainer
				and trainer2 = _trainer2
				and st_class = _st_class 
				and la_class = _la_class 
				and st_score = _st_score 
				and la_score = _la_score 
				and st_la_score = _st_la_score 
				and points = _points 
                and is_archive = _is_archive
			limit 1
		) then
			_need_add = true;
		end if;
	end if;

	if not _need_add then
		_ret = _id;
	else
		insert into dancerlar (op_mode, id 
			, person 
			, external_id
			, club
			, trainer
			, trainer2
			, st_class
			, la_class
			, st_score
			, la_score
			, st_la_score
			, points
            , is_archive
		) 
		values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from dancerlar))
			, _person 
			, _external_id
			, _club
			, _trainer
			, _trainer2
			, _st_class
			, _la_class
			, _st_score
			, _la_score
			, _st_la_score
			, _points
            , _is_archive
		)
		on conflict (id, created_at, op_mode)
		do update set op_mode = excluded.op_mode
			, person = excluded.person 
			, external_id = excluded.external_id
			, club = excluded.club
			, trainer = excluded.trainer
			, trainer2 = excluded.trainer2
			, st_class = excluded.st_class 
			, la_class = excluded.la_class 
			, st_score = excluded.st_score 
			, la_score = excluded.la_score 
			, st_la_score = excluded.st_la_score 
			, points = excluded.points 
			, is_archive = excluded.is_archive
		returning id into _ret;
	end if;

	return _ret;
end;
$$
;

-- CREATE OR REPLACE FUNCTION public.dancerlar(_op_mode smallint, _created_before_or_at timestamp with time zone)
--  RETURNS TABLE(id integer, created_at timestamp with time zone, person integer, external_id integer, club integer, trainer integer, st_class smallint, la_class smallint, st_score integer, la_score integer, st_la_score integer, points integer)
--  LANGUAGE sql
-- AS $function$
-- 	select distinct on(id) id
-- 	, created_at 
-- 	, person 
-- 	, external_id 
-- 	, club 
-- 	, trainer 
-- 	, st_class 
-- 	, la_class 
-- 	, st_score 
-- 	, la_score 
-- 	, st_la_score 
-- 	, points 
-- 	from dancerlar
-- 	where op_mode in (-1, _op_mode)
-- 		and (_created_before_or_at is null or created_at <= _created_before_or_at)
-- 	order by id, created_at desc, op_mode
-- $function$
-- ;

-- select * from export_dancerlar_for_anton(0::smallint);
-- drop function if exists export_dancerlar_for_anton(smallint);
-- create or replace function export_dancerlar_for_anton(_op_mode smallint) 
-- returns table (
-- 	external_id int
-- 	, name text
-- 	, second_name text
-- 	, birth_date date
-- 	, st_class text
-- 	, la_class text
-- 	, club text
-- 	, citi text
-- 	, trainer text
-- 	, region int
-- 	, gender text
-- ) as $$
-- 	select dancerlar.external_id
-- 		, last_name_textlar.value || ' ' || first_name_textlar.value name
-- 		, second_name_textlar.value second_name
-- 		, personlar.birth_date
-- 		, st_classlar.value st_class
-- 		, la_classlar.value la_class
-- 		, club_textlar.value club
-- 		, citi_textlar.value citi
-- 		, trainer_last_name_textlar.value || ' ' || trainer_first_name_textlar.value trainer
-- 		, citilar.region region
-- 		, case 
-- 			when personlar.gender = 1 then 'М'
-- 			when personlar.gender = 2 then 'Ж'
-- 			else null
-- 		end gender
-- 	from (
-- 		select distinct on (external_id) *
-- 		from dancerlar(_op_mode, null) dancerlar
-- 		order by external_id, id desc
-- 	) dancerlar
-- 	--
-- 	join personlar(_op_mode, null) personlar on personlar.id = dancerlar.person 
-- 	join last_namelar(_op_mode, null) last_namelar on last_namelar.id = personlar.last_name
-- 	join textlar last_name_textlar on last_name_textlar.id = last_namelar.value
-- 	join first_namelar(_op_mode, null) first_namelar on first_namelar.id = personlar.first_name
-- 	join textlar first_name_textlar on first_name_textlar.id = first_namelar.value
-- 	join second_namelar(_op_mode, null) second_namelar on second_namelar.id = personlar.second_name
-- 	join textlar second_name_textlar on second_name_textlar.id = second_namelar.value
-- 	--
-- 	join trainerlar(_op_mode, null) trainerlar on trainerlar.id = dancerlar.trainer 
-- 	join personlar(_op_mode, null) trainer_personlar on trainer_personlar.id = trainerlar.person 
-- 	join last_namelar(_op_mode, null) trainer_last_namelar on trainer_last_namelar.id = trainer_personlar.last_name
-- 	join textlar trainer_last_name_textlar on trainer_last_name_textlar.id = trainer_last_namelar.value
-- 	join first_namelar(_op_mode, null) trainer_first_namelar on trainer_first_namelar.id = trainer_personlar.first_name
-- 	join textlar trainer_first_name_textlar on trainer_first_name_textlar.id = trainer_first_namelar.value
-- 	join second_namelar(_op_mode, null) trainer_second_namelar on trainer_second_namelar.id = trainer_personlar.second_name
-- 	join textlar trainer_second_name_textlar on trainer_second_name_textlar.id = trainer_second_namelar.value
-- 	--
-- 	join clublar(_op_mode, null) clublar on clublar.id = dancerlar.club 
-- 	join textlar club_textlar on club_textlar.id = clublar.value
-- 	--
-- 	join citilar(_op_mode, null) citilar on citilar.id = clublar.citi 
-- 	join textlar citi_textlar on citi_textlar.id = citilar.value
-- 	--
-- 	left join classlar st_classlar on st_classlar.id = dancerlar.st_class
-- 	left join classlar la_classlar on la_classlar.id = dancerlar.la_class
-- 	order by last_name_textlar.value
-- 		, first_name_textlar.value
-- 		, second_name_textlar.value
-- $$ language sql;

-- ====================================================================

-- alter table citilar add column region int;
--
-- select c.id, t.value, c.region 
-- from citilar c
-- join textlar t on t.id = c.value;
--
-- update citilar c
-- set region = 77 
-- where (select value from textlar t where t.id = c.value) in ('Москва');
--
-- update citilar c
-- set region = 50 
-- where (select value from textlar t where t.id = c.value) in (
-- 	'Дзержинский',
-- 	'Раменское',
-- 	'Королев',
-- 	'Балашиха',
-- 	'Кубинка'
-- );
--
-- update citilar c
-- set region = 71 
-- where (select value from textlar t where t.id = c.value) in ('Тула');
--
-- update citilar c
-- set region = 78 
-- where (select value from textlar t where t.id = c.value) in ('С.-Петербург');
--
-- update citilar c
-- set region = 47 
-- where (select value from textlar t where t.id = c.value) in (
-- 	'Ленинградская обл.',
-- 	'Всеволожск'
-- );
--
-- update citilar c
-- set region = 39 
-- where (select value from textlar t where t.id = c.value) in ('Калининград');
--
-- update citilar c
-- set region = 32 
-- where (select value from textlar t where t.id = c.value) in ('Брянск');

-- ====================================================================

-- select dancerlar.id "dancerId"
-- 	, last_name_textlar.value "surName"
-- 	, first_name_textlar.value "firstName"
-- 	, second_name_textlar.value "fatherName"
-- 	, case 
-- 		when personlar.gender = 1 then 'm'
-- 		when personlar.gender = 2 then 'f'
-- 		else null
-- 	end
-- 	, dancerlar.external_id "FTSRNumber"
-- 	, personlar.birth_date "BirthDay"
-- 	, dancerlar.club "clubId" 
-- 	, dancerlar.st_class "classStId" 
-- 	, dancerlar.la_class "classLaId" 
-- from (
-- 	select distinct on (external_id) *
-- 	from dancerlar(-1::smallint, null) dancerlar
-- 	order by external_id, id desc
-- ) dancerlar
-- --
-- join personlar(-1::smallint, null) personlar on personlar.id = dancerlar.person 
-- join last_namelar(-1::smallint, null) last_namelar on last_namelar.id = personlar.last_name
-- join textlar last_name_textlar on last_name_textlar.id = last_namelar.value
-- join first_namelar(-1::smallint, null) first_namelar on first_namelar.id = personlar.first_name
-- join textlar first_name_textlar on first_name_textlar.id = first_namelar.value
-- join second_namelar(-1::smallint, null) second_namelar on second_namelar.id = personlar.second_name
-- join textlar second_name_textlar on second_name_textlar.id = second_namelar.value
-- order by dancerlar.id
-- ;
--
-- select dancerlar.id "dancerId"
-- 	, dancerlar.st_class "classStId" 
-- 	, dancerlar.la_class "classLaId" 
-- from (
-- 	select distinct on (external_id) *
-- 	from dancerlar(-1::smallint, null) dancerlar
-- 	order by external_id, id desc
-- ) dancerlar
-- order by dancerlar.id
-- ;
--
-- select dancerlar.id "dancerId"
-- 	, dancerlar.club "clubId" 
-- from (
-- 	select distinct on (external_id) *
-- 	from dancerlar(-1::smallint, null) dancerlar
-- 	order by external_id, id desc
-- ) dancerlar
-- order by dancerlar.id
-- ;


-- select citilar.id, textlar.value 
-- from citilar(-1::smallint, null) citilar
-- join textlar on textlar.id = citilar.value;
--
-- select count(*) from citilar(-1::smallint, null);
--
-- select count(distinct(d.citi)) from dancerlar d;
--
-- select * from classlar c;
--
-- select clublar.id, textlar.value , clublar.citi
-- from clublar(-1::smallint, null) clublar
-- join textlar on textlar.id = clublar.value
-- order by textlar.value
-- ;


-- select *
-- from dancerlar(-1::smallint, null)


-- ==================================================

--psql 'postgres://yb:1014103@localhost/postgres'

--create database a100; -- https://www.postgresql.org/docs/12/sql-createdatabase.html 
--
--create user a100 with password '1014103a100'; -- https://www.postgresql.org/docs/8.0/sql-createuser.html
--
--grant connect on database a100 to a100; -- https://tableplus.com/blog/2018/04/postgresql-how-to-grant-access-to-users.html
--
--grant usage on schema public to a100; 
--
--GRANT ALL PRIVILEGES ON DATABASE a100 TO a100;

--psql 'postgres://a100:1014103a100@localhost/a100'

-- ==================================================
-- ==================================================

--psql 'postgres://admin:nug4Laih@v9z.ru:54495/postgres'
--create database a100; -- https://www.postgresql.org/docs/12/sql-createdatabase.html 
--
--create user a100 with password '1014103a100'; -- https://www.postgresql.org/docs/8.0/sql-createuser.html
--
--grant connect on database a100 to a100; -- https://tableplus.com/blog/2018/04/postgresql-how-to-grant-access-to-users.html
--
--grant usage on schema public to a100; 
--
--GRANT ALL PRIVILEGES ON DATABASE a100 TO a100;

--psql 'postgres://a100:1014103a100@v9z.ru:54495/a100'

-- ==================================================

drop table if exists op_modelar cascade;
create table op_modelar (
	id smallint primary key,
	value text not null unique
);
insert into op_modelar (id, value) values (-1, '')
, (0, 'prod')
, (1, 'dev')
, (2, 'demo')
, (3, 'rc')
, (4, 'local')
;

drop table if exists classlar cascade;
create table classlar (
	id smallint primary key,
	value text not null unique
);
insert into classlar (id, value) values (-1, '')
, (1, 'M')
, (2, 'S')
, (3, 'A')
, (4, 'B')
, (5, 'C')
, (6, 'D')
, (7, 'E')
, (8, 'H5')
, (9, 'H4')
, (10, 'H3')
, (11, 'H2')
on conflict (id) 
do update set value = excluded.value
;
-- select id from classlar c where value = 'H5';

drop table if exists categorilar cascade;
create table categorilar (
	id smallint primary key,
	value text not null unique
);
insert into categorilar (id, value) values (-1, '')
, (0, 'ВК')
, (1, '1К')
, (2, '2К')
, (3, '3К')
, (4, '4К')
, (5, '5К')
, (6, '6К')
, (7, '7К')
on conflict (id) 
do update set value = excluded.value
;

drop table if exists genderlar cascade;
create table genderlar (
	id serial primary key,
	value text not null unique
);
insert into genderlar (value) values ('М'), ('Ж');

-- ==================================================

drop table if exists textlar cascade;
create table textlar (
	id serial primary key,
	value text not null
);
insert into textlar (value) values ('');
create unique index textlar_unique_md5_value_index on textlar (md5(value));
create or replace function add_text(_value text) 
returns int as $$
declare _ret int = 1;
begin
	if _value is not null then
		select into _ret id from textlar where md5(value) = md5(_value);
		if _ret is null then
			insert into textlar (value) 
			values (_value)
			on conflict (md5(value))
			do update set value = excluded.value
			returning id into _ret;
		end if;
	end if;
	return _ret;
end;
$$ language plpgsql;

select add_text('');

-- ==================================================

drop table if exists citilar cascade; 
create table citilar (
	op_mode smallint not null references op_modelar(id) on delete cascade
	, created_at timestamptz not null default now()
	, id int not null
	, value int not null references textlar(id) on delete cascade
	, unique(id, created_at, op_mode)
);
alter table citilar add column region int;
drop function if exists citilar(smallint, timestamptz);
create or replace function citilar(_op_mode smallint, _created_before_or_at timestamptz) 
returns table (
	id int,
	value int,
	region int
)
as $$
	select distinct on(id)
		id, value, region
	from citilar
	where op_mode in (-1, _op_mode) 
		and (_created_before_or_at is null or created_at <= _created_before_or_at)
	order by id, created_at desc, op_mode
$$ language sql;
-- drop function if exists add_citi(int, smallint, text);
create or replace function add_citi(_op_mode smallint, _id int, _value text) 
returns int as $$
declare 
	_ret int;
    _text_id int;
begin
	_text_id = add_text(_value);

	select 
		into _ret
		id
	from citilar(_op_mode, null) 
	where (_id is null or id = _id)
		and value = _text_id  
	-- order by id
	limit 1;

	if _ret is null then
		insert into citilar (op_mode, id, value) 
		values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from citilar))
			, _text_id
		)
		on conflict (op_mode, id, created_at)
		do update set op_mode = excluded.op_mode
			, value = excluded.value
		returning id into _ret;
	end if;

	return _ret;
end;
$$ language plpgsql;

select add_citi(-1::smallint, null, '');

-- select add_citi(-1::smallint, null, 'Москва');
-- select add_citi(4::smallint, null, 'Москва');
-- select add_citi(4::smallint, null, 'Минск');
-- select add_citi(-1::smallint, null, 'Балашиха');
-- select add_citi(4::smallint, 3, 'Балашиха::local');
-- select add_citi(3::smallint, 3, 'Балашиха::rc');
-- select add_citi(2::smallint, 3, 'Балашиха::demo');
-- select add_citi(1::smallint, 3, 'Балашиха::dev');
--
-- -- select c.id, t.value from citilar(4::smallint, null) c join textlar t on t.id = c.value;
-- -- select c.id, t.value from citilar(4::smallint, (select max(created_at) from citilar where id = 3 and op_mode = -1)) c join textlar t on t.id = c.value;
-- -- select c.id, t.value from citilar(3::smallint, null) c join textlar t on t.id = c.value;
-- -- select c.id, t.value from citilar(2::smallint, null) c join textlar t on t.id = c.value;
-- -- select c.id, t.value from citilar(1::smallint, null) c join textlar t on t.id = c.value;
-- -- select c.id, t.value from citilar(0::smallint, null) c join textlar t on t.id = c.value;
-- -- select c.id, t.value from citilar(-1::smallint, null) c join textlar t on t.id = c.value;
-- --
-- -- -- ==================================================
--
drop table if exists first_namelar cascade;
create table first_namelar (
	op_mode smallint not null references op_modelar(id) on delete cascade
	, created_at timestamptz not null default now()
	, id int not null
	, value int not null references textlar(id) on delete cascade
	, default_gender smallint references genderlar(id) -- no cascade delete
    , unique(id, created_at, op_mode)
);
-- drop function if exists first_namelar(smallint, timestamptz);
create or replace function first_namelar(_op_mode smallint, _created_before_or_at timestamptz) 
returns table ( id int
	, value int
	, default_gender smallint
)
as $$
	select distinct on(id)
		id, value, default_gender
	from first_namelar
	where op_mode in (-1, _op_mode)
		and (_created_before_or_at is null or created_at <= _created_before_or_at)
	order by id, created_at desc, op_mode
$$ language sql;
--drop function if exists add_first_name(smallint, int, text);
create or replace function add_first_name(_op_mode smallint, _id int, _value text) 
returns int as $$
declare 
	_ret int;
    _text_id int;
begin
	_text_id = add_text(_value);
	select 
		into _ret
		id
	from first_namelar(_op_mode, null) 
	where (_id is null or id = _id)
		and value = _text_id  
	limit 1;

	if _ret is null then
		insert into first_namelar (op_mode, id, value) 
		values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from first_namelar)) 
			, _text_id
		)
		on conflict (id, created_at, op_mode)
		do update set op_mode = excluded.op_mode
			,  value = excluded.value
		returning id into _ret;
	end if;

	return _ret;
end;
$$ language plpgsql;

select add_first_name(-1::smallint, null::int, '');

-- select add_first_name(-1::smallint, null::int, 'Константин');
-- select add_first_name(4::smallint, 1, 'Константин::local');

-- -- -- ==================================================

drop table if exists second_namelar cascade;
create table second_namelar (
	op_mode smallint not null references op_modelar(id) on delete cascade
	, created_at timestamptz not null default now()
	, id int not null
	, value int not null references textlar(id) on delete cascade
	, default_gender smallint references genderlar(id) -- no cascade delete
    , unique(id, created_at, op_mode)
);
-- drop function if exists second_namelar(smallint, timestamptz);
create or replace function second_namelar(_op_mode smallint, _created_before_or_at timestamptz) 
returns table ( id int
	, value int
	, default_gender smallint
)
as $$
	select distinct on(id)
		id, value, default_gender
	from second_namelar
	where op_mode in (-1, _op_mode)
		and (_created_before_or_at is null or created_at <= _created_before_or_at)
	order by id, created_at desc, op_mode
$$ language sql;
-- drop function if exists add_second_name(int, text, smallint);
create or replace function add_second_name(_op_mode smallint, _id int, _value text) 
returns int as $$
declare 
	_ret int;
    _text_id int;
begin
	_text_id = add_text(_value);
	select 
		into _ret
		id
	from second_namelar(_op_mode, null) 
	where (_id is null or id = _id)
		and value = _text_id  
	limit 1;

	if _ret is null then
		insert into second_namelar (op_mode, id, value) 
		values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from second_namelar)) 
			, _text_id
		)
		on conflict (id, created_at, op_mode)
		do update set op_mode = excluded.op_mode
			,  value = excluded.value
		returning id into _ret;
	end if;

	return _ret;
end;
$$ language plpgsql;

select add_second_name(-1::smallint, null::int, '');

-- select add_second_name(-1::smallint, null::int, 'Константинович');
-- select add_second_name(4::smallint, 1, 'Константинович::local');

-- ==================================================

drop table if exists last_namelar cascade; 
create table last_namelar (
	op_mode smallint not null references op_modelar(id) on delete cascade
	, created_at timestamptz not null default now()
	, id int not null
	, value int not null references textlar(id) on delete cascade
	, unique(id, created_at, op_mode)
);
-- drop function if exists last_namelar(smallint, timestamptz);
create or replace function last_namelar(_op_mode smallint, _created_before_or_at timestamptz) 
returns table (id int
	, value int
)
as $$
	select distinct on(id)
		id, value
	from last_namelar
	where op_mode in (-1, _op_mode) 
		and (_created_before_or_at is null or created_at <= _created_before_or_at)
	order by id, created_at desc, op_mode
$$ language sql;
-- drop function if exists add_last_name(int, text, smallint);
create or replace function add_last_name(_op_mode smallint, _id int, _value text) 
returns int as $$
declare 
	_ret int;
    _text_id int;
begin
	_text_id = add_text(_value);
	select 
		into _ret
		id
	from last_namelar(_op_mode, null) 
	where (_id is null or id = _id)
		and value = _text_id  
	limit 1;

	if _ret is null then
		insert into last_namelar (op_mode, id, value) 
		values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from last_namelar))
			, _text_id
		)
		on conflict (id, created_at, op_mode)
		do update set op_mode = excluded.op_mode
			,  value = excluded.value
		returning id into _ret;
	end if;

	return _ret;
end;
$$ language plpgsql;

select add_last_name(-1::smallint, null::int, '');

-- select add_last_name(-1::smallint, null::int, 'Балашов');
-- select add_last_name(4::smallint, 1, 'Балашов::local');

-- -- -- ==================================================

drop table if exists nick_namelar cascade; 
create table nick_namelar (
	op_mode smallint not null references op_modelar(id) on delete cascade
	, created_at timestamptz not null default now()
	, id int not null
	, value int not null references textlar(id) on delete cascade
	, unique(id, created_at, op_mode)
);
-- drop function if exists nick_namelar(smallint, timestamptz);
create or replace function nick_namelar(_op_mode smallint, _created_before_or_at timestamptz) 
returns table (id int
	, value int
)
as $$
	select distinct on(id)
		id, value
	from nick_namelar
	where op_mode in (-1, _op_mode) 
		and (_created_before_or_at is null or created_at <= _created_before_or_at)
	order by id, created_at desc, op_mode
$$ language sql;
-- drop function if exists add_nick_name(int, text, smallint);
create or replace function add_nick_name(_op_mode smallint, _id int, _value text) 
returns int as $$
declare 
	_ret int;
    _text_id int;
begin
	_text_id = add_text(_value);
	select 
		into _ret
		id
	from nick_namelar(_op_mode, null) 
	where (_id is null or id = _id)
		and value = _text_id  
	limit 1;

	if _ret is null then
		insert into nick_namelar (op_mode, id, value) 
		values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from nick_namelar))
			, _text_id
		)
		on conflict (id, created_at, op_mode)
		do update set op_mode = excluded.op_mode
			, value = excluded.value
		returning id into _ret;
	end if;

	return _ret;
end;
$$ language plpgsql;

select add_nick_name(-1::smallint, null::int, '');

-- ==================================================

drop table if exists personlar cascade;
create table personlar ( 
	op_mode smallint not null references op_modelar(id) on delete cascade
	, created_at timestamptz not null default now()
	, id int not null
	, last_name int not null
	, first_name int not null
	, second_name int not null
	, birth_date date not null default date '1900-01-01'
	, nick_name smallint not null
	, gender smallint references genderlar(id) -- no cascade delete
    , unique(id, created_at, op_mode)
);
-- drop function if exists personlar(smallint, timestamptz);
create or replace function personlar(_op_mode smallint, _created_before_or_at timestamptz) 
returns table ( id int
	, last_name int
	, first_name int
	, second_name int
	, birth_date date
	, nick_name smallint
	, gender smallint 
)
as $$
	select distinct on(id) id
	, last_name
	, first_name
	, second_name
	, birth_date
	, nick_name
	, gender
	from personlar
	where op_mode in (-1, _op_mode)
		and (_created_before_or_at is null or created_at <= _created_before_or_at)
	order by id, created_at desc, op_mode
$$ language sql;
-- drop function if exists add_person(int, text, text, text, text, date, smallint);
create or replace function add_person(_op_mode smallint, _id int
	, _last_name_value text
	, _first_name_value text
	, _second_name_value text
	, _nick_name_value text 
	, _birth_date date
) returns int as $$
declare 
	_ret int;
	_last_name int;
	_first_name int;
	_second_name int;
	_nick_name int;
begin
	_last_name = add_last_name(_op_mode, null, _last_name_value);
	_first_name = add_first_name(_op_mode, null, _first_name_value);
	_second_name = add_second_name(_op_mode, null, _second_name_value);
	_nick_name = add_nick_name(_op_mode, null, _nick_name_value);
	_birth_date = coalesce(_birth_date, '1900-01-01');

	select 
		into _ret
		id
	from personlar(_op_mode, null) 
	where (_id is null or id = _id)
		and last_name = _last_name 
		and first_name = _first_name
		and second_name = _second_name
		and nick_name = _nick_name
		and birth_date = _birth_date
	limit 1;

	if _ret is null then
		insert into personlar (op_mode, id
			, last_name
			, first_name
			, second_name
			, nick_name
			, birth_date
		) 
		values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from personlar))
			, _last_name
			, _first_name
			, _second_name
			, _nick_name
			, _birth_date
		)
		on conflict (id, created_at, op_mode)
		do update set op_mode = excluded.op_mode
			, last_name = excluded.last_name 
			, first_name = excluded.first_name
			, second_name = excluded.second_name
			, nick_name = excluded.nick_name
			, birth_date = excluded.birth_date
		returning id into _ret;
	end if;

	return _ret;
end;
$$ language plpgsql;

-- select add_person(-1::smallint, null::int, 'Соколов', 'Александр', 'Сергеевич', '', null::date);

-- ==================================================

drop table if exists clublar cascade;
create table clublar (
	op_mode smallint not null references op_modelar(id) on delete cascade
	, created_at timestamptz not null default now()
	, id int not null
	, value int not null references textlar(id) on delete cascade
	, citi int not null 
	, chief int 
	, unique(id, created_at, op_mode)
);
drop function if exists clublar(smallint, timestamptz);
create or replace function clublar(_op_mode smallint, _created_before_or_at timestamptz) 
returns table ( id int
	, value int
	, citi int
	, chief int
)
as $$
	select distinct on(id)
		id, value, citi, chief
	from clublar
	where op_mode in (-1, _op_mode)
		and (_created_before_or_at is null or created_at <= _created_before_or_at)
	order by id, created_at desc, op_mode
$$ language sql;
-- drop function if exists add_club(int, text, text, smallint); 
create or replace function add_club(_op_mode smallint
	, _id int
	, _value text
	, _citi int
    , _chief int
) 
returns int as $$
declare 
	_ret int;
    _text_id int;
	_need_add bool = false;
begin
	_text_id = add_text(_value);

	if _id is null then
		select 
			into _id
			id
		from clublar(_op_mode, null) 
		where value = _text_id and citi = _citi
		limit 1;
	end if;

	if _id is null then
		_need_add = true;
	else 
		if not exists (
			select id
            from clublar(_op_mode, null) 
			where id = _id
                and value = _text_id  
                and citi = _citi
                and (chief = _chief or _chief is null)
			limit 1
		) then
			_need_add = true;
		end if;
	end if;

	if not _need_add then
		_ret = _id;
	else
		if _chief is null then
			insert into clublar (op_mode, id, value, citi) 
			values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from clublar)) 
				, _text_id
				, _citi
			)
			on conflict (id, created_at, op_mode)
			do update set op_mode = excluded.op_mode
				, value = excluded.value
				, citi = excluded.citi
			returning id into _ret;
		else
			insert into clublar (op_mode, id, value, citi, chief) 
			values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from clublar)) 
				, _text_id
				, _citi
				, _chief
			)
			on conflict (id, created_at, op_mode)
			do update set op_mode = excluded.op_mode
				, value = excluded.value
				, citi = excluded.citi
				, chief = excluded.chief
			returning id into _ret;
		end if;
	end if;

	return _ret;
end;
$$ language plpgsql;

-- select add_club(-1::smallint, null::int, '', '');
-- select add_club(-1::smallint, 5::int, 'Нева', 'С.-Петербург');
--  select add_club(-1::smallint, null::int, 'Мечта'::text, 
--  	add_citi(-1::smallint, null::int, 'Тула'::int,
--  	null::int
--  );

-- create or replace function add_club(_op_mode smallint
--     , _id int
--     , _value text
--     , _citi_value text
--     , _chief int
--     -- , _chief_last_name text
--     -- , _chief_first_name text
--     -- , _chief_second_name text
--     -- , _chief_nick_name text
-- ) 
-- returns int as $$
-- declare 
-- 	_ret int;
--     _text_id int;
-- 	_citi int;
-- 	_need_add bool = false;
-- begin
-- 	_text_id = add_text(_value);
-- 	_citi = add_citi(_op_mode, _id, _citi_value);
--
-- 	select 
-- 		into _id
-- 		id
-- 	from clublar(_op_mode, null) 
-- 	where id = _id
-- 	limit 1;
-- 	if _id is null then
-- 		_need_add = true;
-- 	else 
-- 		if not exists (
-- 			select id
-- 			from clublar(_op_mode, null) 
-- 			where id = _id
--                 and value = _text_id  
--                 and citi = _citi
--                 and chief = _chief
-- 			limit 1
-- 		) then
-- 			_need_add = true;
-- 		end if;
-- 	end if;
--
-- 	if _ret is null then
-- 		insert into clublar (op_mode, id, value, citi, chief) 
-- 		values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from clublar)) 
-- 			, _text_id
-- 			, _citi
--             , _chief
-- 		)
-- 		on conflict (id, created_at, op_mode)
-- 		do update set op_mode = excluded.op_mode
-- 			, value = excluded.value
-- 			, citi = excluded.citi
--             , chief = excluded.chief
-- 		returning id into _ret;
-- 	end if;
--
-- 	return _ret;
-- end;
-- $$ language plpgsql;

-- select add_club(-1::smallint, null::int, '', '');
-- select add_club(-1::smallint
--     , 5::int
--     , 'Нева'::text
--     , 'С.-Петербург'::text
--     , add_person(-1::smallint
--         , null::int
--         , 'Андрейченко'::text
--         , 'Андрей'::text
--         , 'Николаевич'::text
--         , ''::text, null::date
--     )::int
-- );



-- select add_club(-1::smallint, null::int, '10 ритмов', 'Раменское');
-- select add_club(-1::smallint, null::int, 'АЛС', 'Москва');
-- select add_club(4::smallint, null::int, 'Авалон', 'Москва');
-- select add_club(-1::smallint, null::int, 'Ониона-М', 'Балашиха');
--
-- select c.id, t.value, t2.value as citi
-- from clublar(4::smallint, null) c 
-- join textlar t on t.id = c.value
-- join citilar(4::smallint, null) c2 on c2.id = c.citi
-- join textlar t2 on t2.id = c2.value
-- order by t.value
-- ;
--
-- select c.id, t.value, coalesce(t2.value, t3.value) as citi
-- from clublar(4::smallint, null) c 
-- join textlar t on t.id = c.value
-- left join citilar(4::smallint, (select max(created_at) from citilar where id = 3 and op_mode = -1)) c2 on c2.id = c.citi
-- left join textlar t2 on t2.id = c2.value
-- join citilar(4::smallint, null) c3 on c3.id = c.citi
-- join textlar t3 on t3.id = c3.value
-- order by t.value
-- ;
-- select c.id, t.value, t2.value as citi
-- from clublar(0::smallint, null) c 
-- join textlar t on t.id = c.value
-- join citilar(0::smallint, null) c2 on c2.id = c.citi
-- join textlar t2 on t2.id = c2.value
-- ;
--
-- -- -- ==================================================

drop table if exists judgelar cascade;
create table judgelar ( 
	op_mode smallint not null references op_modelar(id) on delete cascade
	, created_at timestamptz not null default now()
	, id int not null
	, person int not null
	, external_id int not null default 0
	, categori smallint not null references categorilar(id)
	, assignment_date date not null default '1900-01-01'
	, club int not null 
	, number_of_participation_in_festivals int not null
	, is_archive bool not null default false
    , unique(id, created_at, op_mode)
);
-- alter table judgelar add column is_archive bool not null default false;
drop function if exists judgelar(smallint, timestamptz);
create or replace function judgelar(_op_mode smallint, _created_before_or_at timestamptz) 
returns table ( id int
	, person int
	, external_id int
	, categori smallint
	, assignment_date date
	, club int
	, number_of_participation_in_festivals int 
	, is_archive bool 
)
as $$
	select distinct on(id) id
	, person 
	, external_id 
	, categori 
	, assignment_date 
	, club 
	, number_of_participation_in_festivals 
	, is_archive 
	from judgelar
	where op_mode in (-1, _op_mode)
		and (_created_before_or_at is null or created_at <= _created_before_or_at)
	order by id, created_at desc, op_mode
$$ language sql;

-- drop function if exists add_judge(int, int, int, smallint, date, int, int, smallint);
create or replace function add_judge(_op_mode smallint, _id int
	, _person int
	, _external_id int
	, _categori smallint 
	, _assignment_date date
	, _club int
	, _number_of_participation_in_festivals int
) returns int as $$
declare 
	_ret int;
	_need_add bool = false;
begin
	_assignment_date = coalesce(_assignment_date, '1900-01-01');
	_external_id = coalesce(_external_id, 0);

	select 
		into _id
		id
	from judgelar(_op_mode, null) 
	where (_id is null or id = _id)
		and (
			_external_id != 0 and external_id = _external_id 
			or 
			_external_id = 0 and person = _person
		)
	limit 1;
	if _id is null then
		_need_add = true;
	else 
		if not exists (
			select id
			from judgelar(_op_mode, null) 
			where id = _id
				and person = _person 
				and external_id = _external_id 
				and club = _club
                and categori = _categori
                and assignment_date = _assignment_date
                and club = _club
                and number_of_participation_in_festivals = _number_of_participation_in_festivals
			limit 1
		) then
			_need_add = true;
		end if;
	end if;

	-- select 
	-- 	into _ret
	-- 	id
	-- from judgelar(_op_mode, null) 
	-- where (_id is null or id = _id)
	-- 	and person = _person 
	-- 	and external_id = _external_id 
	-- 	and categori = _categori
	-- 	and assignment_date = _assignment_date
	-- 	and club = _club
	-- 	and number_of_participation_in_festivals = _number_of_participation_in_festivals
	-- limit 1;

	if not _need_add then
		_ret = _id;
	else
		insert into judgelar (op_mode, id 
			, person 
			, external_id
			, categori 
			, assignment_date 
			, club 
			, number_of_participation_in_festivals 
		) 
		values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from judgelar))
			, _person 
			, _external_id
			, _categori 
			, _assignment_date 
			, _club 
			, _number_of_participation_in_festivals 
		)
		on conflict (id, created_at, op_mode)
		do update set op_mode = excluded.op_mode
			, person = excluded.person 
			, external_id = excluded.external_id
			, categori = excluded.categori
			, assignment_date = excluded.assignment_date
			, club = excluded.club
			, number_of_participation_in_festivals = excluded.number_of_participation_in_festivals
		returning id into _ret;
	end if;

	return _ret;
end;
$$ language plpgsql;

create or replace function add_judge(_op_mode smallint, _id int
	, _person int
	, _external_id int
	, _categori smallint 
	, _assignment_date date
	, _club int
	, _number_of_participation_in_festivals int
    , _is_archive bool
) returns int as $$
declare 
	_ret int;
	_need_add bool = false;
begin
	_assignment_date = coalesce(_assignment_date, '1900-01-01');
	_external_id = coalesce(_external_id, 0);
    _is_archive = coalesce(_is_archive, false);

	if _id is null then
		select 
			into _id
			id
		from judgelar(_op_mode, null) 
		where false
			or _external_id != 0 and external_id = _external_id 
			or _external_id = 0 and person = _person
		limit 1;
	end if;

	if _id is null then
		_need_add = true;
	else 
		if not exists (
			select id
			from judgelar(_op_mode, null) 
			where id = _id
				and person = _person 
				and external_id = _external_id 
				and club = _club
                and categori = _categori
                and assignment_date = _assignment_date
                and club = _club
                and number_of_participation_in_festivals = _number_of_participation_in_festivals
                and is_archive = _is_archive
			limit 1
		) then
			_need_add = true;
		end if;
	end if;

	if not _need_add then
		_ret = _id;
	else
		insert into judgelar (op_mode, id 
			, person 
			, external_id
			, categori 
			, assignment_date 
			, club 
			, number_of_participation_in_festivals 
            , is_archive
		) 
		values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from judgelar))
			, _person 
			, _external_id
			, _categori 
			, _assignment_date 
			, _club 
			, _number_of_participation_in_festivals 
            , _is_archive
		)
		on conflict (id, created_at, op_mode)
		do update set op_mode = excluded.op_mode
			, person = excluded.person 
			, external_id = excluded.external_id
			, categori = excluded.categori
			, assignment_date = excluded.assignment_date
			, club = excluded.club
			, number_of_participation_in_festivals = excluded.number_of_participation_in_festivals
			, is_archive = excluded.is_archive
		returning id into _ret;
	end if;

	return _ret;
end;
$$ language plpgsql;


-- select add_judge(-1::smallint
-- 	, null::int
-- 	, add_person(-1::smallint, null::int, 'Соколов', 'Александр', 'Сергеевич', '', null::date)
-- 	, 38
-- 	, (select id from categorilar where value = '2К')
-- 	, null -- date '2015-12-20',
-- 	, add_club(-1::smallint, null::int, 'ДК Прожектор', 'Москва')
-- 	, 2
-- ) id;

-- ==================================================

drop table if exists trainerlar cascade;

create table trainerlar ( id int not null
	, created_at timestamptz not null default now()
	, op_mode smallint not null references op_modelar(id) on delete cascade
	, person int not null
	, club int not null 
    , unique(id, created_at, op_mode)
);
-- drop function if exists trainerlar(smallint, timestamptz);
create or replace function trainerlar(_op_mode smallint, _created_before_or_at timestamptz) 
returns table ( id int
	, person int
	, club int
)
as $$
	select distinct on(id) id
	, person 
	, club 
	from trainerlar
	where op_mode in (-1, _op_mode)
		and (_created_before_or_at is null or created_at <= _created_before_or_at)
	order by id, created_at desc, op_mode
$$ language sql;
-- drop function if exists add_trainer(int, int, int, smallint, date, int, int, smallint);
-- create or replace function add_trainer( _op_mode smallint, _id int
-- 	, _last_name_value text
-- 	, _first_name_value text
-- 	, _club_value text
-- 	, _citi_value text
-- ) returns int as $$
-- declare 
-- 	_ret int;
-- 	_person int;
-- 	_club int;
-- begin
-- 	select 
-- 		into _person, _club 
-- 		personlar.id, clublar.id
-- 	from judgelar(-1::smallint, null) judgelar
-- 	join personlar(-1::smallint, null) personlar on personlar.id = judgelar.person 
-- 	join last_namelar(-1::smallint, null) last_namelar on last_namelar.id = personlar.last_name
-- 	join textlar last_namelar_textlar on last_namelar_textlar.id = last_namelar.value
-- 	join first_namelar(-1::smallint, null) first_namelar on first_namelar.id = personlar.first_name
-- 	join textlar first_name_textlar on first_name_textlar.id = first_namelar.value
-- 	join clublar(-1::smallint, null) clublar on clublar.id = judgelar.club
-- 	join textlar club_textlar on club_textlar.id = clublar.value
-- 	join citilar(-1::smallint, null) citilar on citilar.id = clublar.citi
-- 	join textlar citi_textlar on citi_textlar.id = citilar.value
-- 	where true 
-- 		and last_namelar_textlar.value = _last_name_value
-- 		and first_name_textlar.value  = _first_name_value
-- 		and club_textlar.value = _club_value
-- 		and citi_textlar.value = _citi_value
-- 	;
--
-- 	if _person is null then
-- 		_person = add_person(-1::smallint, null::int, _last_name_value, _first_name_value, '', '', null::date);
-- 	end if;
-- 	if _club is null then
-- 		_club = add_club(_op_mode, null::int, _club_value, _citi_value);
-- 	end if;
--
-- 	select 
-- 		into _ret
-- 		id
-- 	from trainerlar(_op_mode, null) 
-- 	where (_id is null or id = _id)
-- 		and person = _person 
-- 		and club = _club
-- 	limit 1;
--
-- 	if _ret is null then
-- 		insert into trainerlar (op_mode, id 
-- 			, person 
-- 			, club 
-- 		) 
-- 		values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from trainerlar)) 
-- 			, _person 
-- 			, _club 
-- 		)
-- 		on conflict (id, created_at, op_mode)
-- 		do update set op_mode = excluded.op_mode
-- 			, person = excluded.person 
-- 			, club = excluded.club
-- 		returning id into _ret;
-- 	end if;
--
-- 	return _ret;
-- end;
-- $$ language plpgsql;

create or replace function add_trainer( _op_mode smallint, _id int
	, _last_name_value text
	, _first_name_value text
	, _second_name_value text
	, _nick_name_value text
	, _club int
) returns int as $$
declare 
	_ret int;
	_person int;
	_need_add bool = false;
begin
	select 
		into _person
		personlar.id
	from trainerlar(-1::smallint, null) trainerlar
	join personlar(-1::smallint, null) personlar on personlar.id = trainerlar.person 
	join last_namelar(-1::smallint, null) last_namelar on last_namelar.id = personlar.last_name
	join textlar last_namelar_textlar on last_namelar_textlar.id = last_namelar.value
	join first_namelar(-1::smallint, null) first_namelar on first_namelar.id = personlar.first_name
	join textlar first_name_textlar on first_name_textlar.id = first_namelar.value
	join second_namelar(-1::smallint, null) second_namelar on second_namelar.id = personlar.second_name
	join textlar second_name_textlar on second_name_textlar.id = second_namelar.value
	join nick_namelar(-1::smallint, null) nick_namelar on nick_namelar.id = personlar.nick_name
	join textlar nick_name_textlar on nick_name_textlar.id = nick_namelar.value
    where true 
		and last_namelar_textlar.value = _last_name_value
		and first_name_textlar.value  = _first_name_value
		and (second_name_textlar.value  = _second_name_value or _second_name_value = '')
		and (nick_name_textlar.value  = _nick_name_value or _nick_name_value = '')
        and trainerlar.club = _club
	;
    if _person is null then
        select 
            into _person
            personlar.id
        from judgelar(-1::smallint, null) judgelar
        join personlar(-1::smallint, null) personlar on personlar.id = judgelar.person 
        join last_namelar(-1::smallint, null) last_namelar on last_namelar.id = personlar.last_name
        join textlar last_namelar_textlar on last_namelar_textlar.id = last_namelar.value
        join first_namelar(-1::smallint, null) first_namelar on first_namelar.id = personlar.first_name
        join textlar first_name_textlar on first_name_textlar.id = first_namelar.value
        join second_namelar(-1::smallint, null) second_namelar on second_namelar.id = personlar.second_name
        join textlar second_name_textlar on second_name_textlar.id = second_namelar.value
        join nick_namelar(-1::smallint, null) nick_namelar on nick_namelar.id = personlar.nick_name
        join textlar nick_name_textlar on nick_name_textlar.id = nick_namelar.value
        where true 
            and last_namelar_textlar.value = _last_name_value
            and first_name_textlar.value  = _first_name_value
            and (second_name_textlar.value  = _second_name_value or _second_name_value = '')
            and (nick_name_textlar.value  = _nick_name_value or _nick_name_value = '')
            and judgelar.club = _club
        ;
    end if;

	if _person is null then
		_person = add_person(-1::smallint
			, null::int
			, _last_name_value
			, _first_name_value
			, _second_name_value
			, _nick_name_value
			, null::date
		);
	end if;

	if _id is null then
		select 
			into _id
			id
		from trainerlar(_op_mode, null) 
		where person = _person 
		limit 1;
	end if;

	if _id is null then
		_need_add = true;
	else 
		if not exists (
			select id
			from trainerlar(_op_mode, null) 
			where id = _id
				and person = _person 
				and club = _club 
			limit 1
		) then
			_need_add = true;
		end if;
	end if;

	if not _need_add then
		_ret = _id;
	else
		insert into trainerlar (op_mode, id 
			, person 
			, club 
		) 
		values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from trainerlar)) 
			, _person 
			, _club 
		)
		on conflict (id, created_at, op_mode)
		do update set op_mode = excluded.op_mode
			, person = excluded.person 
			, club = excluded.club
		returning id into _ret;
	end if;

	return _ret;
end;
$$ language plpgsql;

-- 	select 
-- 		personlar.id
-- 	from trainerlar(-1::smallint, null) trainerlar
-- 	join personlar(-1::smallint, null) personlar on personlar.id = trainerlar.person 
-- 	join last_namelar(-1::smallint, null) last_namelar on last_namelar.id = personlar.last_name
-- 	join textlar last_namelar_textlar on last_namelar_textlar.id = last_namelar.value
-- 	join first_namelar(-1::smallint, null) first_namelar on first_namelar.id = personlar.first_name
-- 	join textlar first_name_textlar on first_name_textlar.id = first_namelar.value
-- 	join second_namelar(-1::smallint, null) second_namelar on second_namelar.id = personlar.second_name
-- 	join textlar second_name_textlar on second_name_textlar.id = second_namelar.value
-- 	join nick_namelar(-1::smallint, null) nick_namelar on nick_namelar.id = personlar.nick_name
-- 	join textlar nick_name_textlar on nick_name_textlar.id = nick_namelar.value
--     where true 
-- 		and last_namelar_textlar.value = 'Горбачева'
-- 		and first_name_textlar.value  = 'Ольга'
-- 		and (second_name_textlar.value  = '' or '' = '')
-- 		and (nick_name_textlar.value  = '' or '' = '')
--         and trainerlar.club = 1
-- 	;
--
-- select add_trainer(-1::smallint
--     , null::int
--     , 'Горбачева'
--     , 'Ольга'
--     , ''
--     , ''
--     , add_club(-1::smallint, null::int, 'Фестиваль', 'Москва')::int
-- ) trainer_id;




-- select add_trainer(-1::smallint, null::int, 'Горбачева', 'Ольга', 'Фестиваль', 'Москва') trainer_id;
-- select add_trainer(-1::smallint, null::int, 'Соколов', 'Александр', 'ДК Прожектор', 'Москва') trainer_id;

-- ==================================================

drop table if exists dancerlar cascade;
create table dancerlar ( 
	op_mode smallint not null references op_modelar(id) on delete cascade
	, created_at timestamptz not null default now()
	, id int not null
	, person int not null
	, external_id int not null default 0
	, club int not null 
	, trainer int not null 
	, trainer2 int 
	, st_class smallint not null references classlar(id) -- no cascade delete
	, la_class smallint not null references classlar(id) -- no cascade delete
	, st_score int not null default 0
	, la_score int not null default 0
	, st_la_score int not null default 0
	, points int not null default 0
	, is_archive bool not null default false
    , unique(id, created_at, op_mode)
);
drop function if exists dancerlar(smallint, timestamptz);
create or replace function dancerlar(_op_mode smallint, _created_before_or_at timestamptz) 
returns table ( id int
	, created_at timestamptz
	, person int
	, external_id int
	, club int
	, trainer int 
	, trainer2 int 
	, st_class smallint 
	, la_class smallint 
	, st_score int 
	, la_score int 
	, st_la_score int 
	, points int 
	, is_archive bool
)
as $$
	select distinct on(id) id
	, created_at 
	, person 
	, external_id 
	, club 
	, trainer 
	, trainer2 
	, st_class 
	, la_class 
	, st_score 
	, la_score 
	, st_la_score 
	, points 
	, is_archive
	from dancerlar
	where op_mode in (-1, _op_mode)
		and (_created_before_or_at is null or created_at <= _created_before_or_at)
	order by id, created_at desc, op_mode
$$ language sql;
-- drop function if exists add_dancer(int, int, int, smallint, date, int, int, smallint);
-- create or replace function add_dancer(_op_mode smallint, _id int
-- 	, _person int
-- 	, _external_id int
-- 	, _club int
-- 	, _trainer int 
-- 	, _st_class smallint 
-- 	, _la_class smallint 
-- 	, _st_score int 
-- 	, _la_score int 
-- 	, _st_la_score int 
-- 	, _points int 
-- ) returns int as $$
-- declare 
-- 	_ret int;
-- begin
-- 	_external_id = coalesce(_external_id, 0);
-- 	_st_score = coalesce(_st_score, 0); 
-- 	_la_score = coalesce(_la_score, 0); 
-- 	_st_la_score = coalesce(_st_la_score, 0); 
-- 	_points = coalesce(_points, 0); 
--
-- 	select 
-- 		into _ret
-- 		id
-- 	from dancerlar(_op_mode, null) 
-- 	where (_id is null or id = _id)
-- 		and person = _person 
-- 		and external_id = _external_id 
-- 		and club = _club
-- 		and trainer = _trainer
-- 		and st_class = _st_class 
-- 		and la_class = _la_class 
-- 		and st_score = _st_score 
-- 		and la_score = _la_score 
-- 		and st_la_score = _st_la_score 
-- 		and points = _points 
-- 	limit 1;
--
-- 	if _ret is null then
-- 		insert into dancerlar (op_mode, id 
-- 			, person 
-- 			, external_id
-- 			, club
-- 			, trainer
-- 			, st_class
-- 			, la_class
-- 			, st_score
-- 			, la_score
-- 			, st_la_score
-- 			, points
-- 		) 
-- 		values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from dancerlar))
-- 			, _person 
-- 			, _external_id
-- 			, _club
-- 			, _trainer
-- 			, _st_class
-- 			, _la_class
-- 			, _st_score
-- 			, _la_score
-- 			, _st_la_score
-- 			, _points
-- 		)
-- 		on conflict (id, created_at, op_mode)
-- 		do update set op_mode = excluded.op_mode
-- 			, person = excluded.person 
-- 			, external_id = excluded.external_id
-- 			, club = excluded.club
-- 			, trainer = excluded.trainer
-- 			, st_class = excluded.st_class 
-- 			, la_class = excluded.la_class 
-- 			, st_score = excluded.st_score 
-- 			, la_score = excluded.la_score 
-- 			, st_la_score = excluded.st_la_score 
-- 			, points = excluded.points 
-- 		returning id into _ret;
-- 	end if;
--
-- 	return _ret;
-- end;
-- $$ language plpgsql;

-- select add_dancer( -1::smallint
-- 	, null::int
-- 	, add_person(-1::smallint
-- 		, null::int
-- 		, 'Касперович'::text
-- 		, 'Варвара'::text
-- 		, 'Юрьевна'::text
-- 		, ''::text
-- 		, '2010-02-05'::date
-- 	)::int
-- 	, 5500001::int
-- 	, add_club(-1::smallint, null::int, 'Фестиваль', 'Москва')::int
-- 	, add_trainer(-1::smallint, null::int, 'Горбачева', 'Ольга', 'Фестиваль', 'Москва')
-- 	, (select id from classlar where value = 'E')
-- 	, (select id from classlar where value = 'E')
-- 	, 0::int
-- 	, 0::int
-- 	, 0::int
-- 	, 0::int
-- );

-- select add_dancer(-1::smallint
-- 	, null::int
-- 	, add_person(-1::smallint, null::int, 'Опарина', 'Надежда', 'Ильинична', '', date '2002-12-25')
-- 	, 5500163
-- 	, add_club(-1::smallint, null::int, 'ДК Прожектор', 'Москва')
-- 	, add_trainer(-1::smallint, null::int, 'Соколов', 'Александр', 'ДК Прожектор', 'Москва')
-- 	, (select id from classlar where value = 'C')
-- 	, (select id from classlar where value = 'C')
-- 	, 0
-- 	, 0
-- 	, 0
-- 	, 0
-- );

-- ==================================================

drop function if exists export_judgelar(smallint);
create or replace function export_judgelar(_op_mode smallint) 
returns table (id int
	, external_id int
	, last_name text
	, first_name text
	, second_name text
	, nick_name text
	, categori text
	, assignment_date date
	, club text
	, citi text
	, number_of_participation_in_festivals int
	, is_archive bool
) as $$
	select judgelar.id
		, judgelar.external_id
		, last_name_textlar.value last_name
		, first_name_textlar.value first_name
		, second_name_textlar.value second_name
		, nick_name_textlar.value nick_name
		, categorilar.value categori
		, judgelar.assignment_date
		, club_textlar.value club
		, citi_textlar.value citi
		, judgelar.number_of_participation_in_festivals
		, judgelar.is_archive
	from judgelar(_op_mode, null) judgelar
	--
	join personlar(_op_mode, null) personlar on personlar.id = judgelar.person 
	join last_namelar(_op_mode, null) last_namelar on last_namelar.id = personlar.last_name
	join textlar last_name_textlar on last_name_textlar.id = last_namelar.value
	join first_namelar(_op_mode, null) first_namelar on first_namelar.id = personlar.first_name
	join textlar first_name_textlar on first_name_textlar.id = first_namelar.value
	join second_namelar(_op_mode, null) second_namelar on second_namelar.id = personlar.second_name
	join textlar second_name_textlar on second_name_textlar.id = second_namelar.value
	join nick_namelar(_op_mode, null) nick_namelar on nick_namelar.id = personlar.nick_name
	join textlar nick_name_textlar on nick_name_textlar.id = nick_namelar.value
	--
	left join categorilar on categorilar.id = judgelar.categori
	--
	join clublar(_op_mode, null) clublar on clublar.id = judgelar.club 
	join textlar club_textlar on club_textlar.id = clublar.value
	--
	join citilar(_op_mode, null) citilar on citilar.id = clublar.citi 
	join textlar citi_textlar on citi_textlar.id = citilar.value
	order by judgelar.external_id
		, last_name_textlar.value 
		, first_name_textlar.value 
		, second_name_textlar.value 
		, nick_name_textlar.value 
$$ language sql;

-- select * from export_judgelar(0::smallint);

drop function if exists export_clublar(smallint);
create or replace function export_clublar(_op_mode smallint) 
returns table (id int
	, club text
	, citi text
	, chief_last_name text
	, chief_first_name text
	, chief_second_name text
	, chief_nick_name text
) as $$
	select clublar.id
		, club_textlar.value club
		, citi_textlar.value citi
		, last_name_textlar.value chief_last_name
		, first_name_textlar.value chief_first_name
		, second_name_textlar.value chief_second_name
		, nick_name_textlar.value chief_nick_name
	from clublar(_op_mode, null) clublar
	--
	left join personlar(_op_mode, null) personlar on personlar.id = clublar.chief 
	left join last_namelar(_op_mode, null) last_namelar on last_namelar.id = personlar.last_name
	left join textlar last_name_textlar on last_name_textlar.id = last_namelar.value
	left join first_namelar(_op_mode, null) first_namelar on first_namelar.id = personlar.first_name
	left join textlar first_name_textlar on first_name_textlar.id = first_namelar.value
	left join second_namelar(_op_mode, null) second_namelar on second_namelar.id = personlar.second_name
	left join textlar second_name_textlar on second_name_textlar.id = second_namelar.value
	left join nick_namelar(_op_mode, null) nick_namelar on nick_namelar.id = personlar.nick_name
	left join textlar nick_name_textlar on nick_name_textlar.id = nick_namelar.value
	--
	join textlar club_textlar on club_textlar.id = clublar.value
	--
	join citilar(_op_mode, null) citilar on citilar.id = clublar.citi 
	join textlar citi_textlar on citi_textlar.id = citilar.value
	where club_textlar.value != ''
	order by club_textlar.value, citi_textlar.value
$$ language sql;

-- select * from export_clublar(0::smallint);


drop function if exists export_dancerlar(smallint);
create or replace function export_dancerlar(_op_mode smallint) 
returns table (id int
	, external_id int
	, last_name text
	, first_name text
	, second_name text
	, nick_name text
	, birth_date date
	, trainer_last_name text
	, trainer_first_name text
	, trainer_second_name text
	, trainer_nick_name text
	, trainer2_last_name text
	, trainer2_first_name text
	, trainer2_second_name text
	, trainer2_nick_name text
	, club text
	, citi text
	, st_class text
	, la_class text
	, st_score double precision
	, la_score double precision
	, st_la_score double precision
	, points double precision
	, is_archive bool
) as $$
	select dancerlar.id
		, dancerlar.external_id
		, last_name_textlar.value last_name
		, first_name_textlar.value first_name
		, second_name_textlar.value second_name
		, nick_name_textlar.value nick_name
		, personlar.birth_date
		, trainer_last_name_textlar.value trainer_last_name
		, trainer_first_name_textlar.value trainer_first_name
		, trainer_second_name_textlar.value trainer_second_name
		, trainer_nick_name_textlar.value trainer_nick_name
		, trainer2_last_name_textlar.value trainer2_last_name
		, trainer2_first_name_textlar.value trainer2_first_name
		, trainer2_second_name_textlar.value trainer2_second_name
		, trainer2_nick_name_textlar.value trainer2_nick_name
		, club_textlar.value club
		, citi_textlar.value citi
		, st_classlar.value st_class
		, la_classlar.value la_class
		, (dancerlar.st_score::double precision) / 4 st_score
		, (dancerlar.la_score::double precision) / 4 la_score
		, (dancerlar.st_la_score::double precision) / 4 st_la_score
		, (dancerlar.points::double precision)/ 10 points
		, dancerlar.is_archive
	from (
		select distinct on (external_id) *
		from dancerlar(_op_mode, null) dancerlar
		order by external_id, id desc
	) dancerlar
	--
	join personlar(_op_mode, null) personlar on personlar.id = dancerlar.person 
	join last_namelar(_op_mode, null) last_namelar on last_namelar.id = personlar.last_name
	join textlar last_name_textlar on last_name_textlar.id = last_namelar.value
	join first_namelar(_op_mode, null) first_namelar on first_namelar.id = personlar.first_name
	join textlar first_name_textlar on first_name_textlar.id = first_namelar.value
	join second_namelar(_op_mode, null) second_namelar on second_namelar.id = personlar.second_name
	join textlar second_name_textlar on second_name_textlar.id = second_namelar.value
	join nick_namelar(_op_mode, null) nick_namelar on nick_namelar.id = personlar.nick_name
	join textlar nick_name_textlar on nick_name_textlar.id = nick_namelar.value
	--
	join trainerlar(_op_mode, null) trainerlar on trainerlar.id = dancerlar.trainer 
	join personlar(_op_mode, null) trainer_personlar on trainer_personlar.id = trainerlar.person 
	join last_namelar(_op_mode, null) trainer_last_namelar on trainer_last_namelar.id = trainer_personlar.last_name
	join textlar trainer_last_name_textlar on trainer_last_name_textlar.id = trainer_last_namelar.value
	join first_namelar(_op_mode, null) trainer_first_namelar on trainer_first_namelar.id = trainer_personlar.first_name
	join textlar trainer_first_name_textlar on trainer_first_name_textlar.id = trainer_first_namelar.value
	join second_namelar(_op_mode, null) trainer_second_namelar on trainer_second_namelar.id = trainer_personlar.second_name
	join textlar trainer_second_name_textlar on trainer_second_name_textlar.id = trainer_second_namelar.value
	join nick_namelar(_op_mode, null) trainer_nick_namelar on trainer_nick_namelar.id = trainer_personlar.nick_name
	join textlar trainer_nick_name_textlar on trainer_nick_name_textlar.id = trainer_nick_namelar.value
	--
	left join trainerlar(_op_mode, null) trainerlar2 on trainerlar2.id = dancerlar.trainer2 
	left join personlar(_op_mode, null) trainer2_personlar on trainer2_personlar.id = trainerlar2.person 
	left join last_namelar(_op_mode, null) trainer2_last_namelar on trainer2_last_namelar.id = trainer2_personlar.last_name
	left join textlar trainer2_last_name_textlar on trainer2_last_name_textlar.id = trainer2_last_namelar.value
	left join first_namelar(_op_mode, null) trainer2_first_namelar on trainer2_first_namelar.id = trainer2_personlar.first_name
	left join textlar trainer2_first_name_textlar on trainer2_first_name_textlar.id = trainer2_first_namelar.value
	left join second_namelar(_op_mode, null) trainer2_second_namelar on trainer2_second_namelar.id = trainer2_personlar.second_name
	left join textlar trainer2_second_name_textlar on trainer2_second_name_textlar.id = trainer2_second_namelar.value
	left join nick_namelar(_op_mode, null) trainer2_nick_namelar on trainer2_nick_namelar.id = trainer2_personlar.nick_name
	left join textlar trainer2_nick_name_textlar on trainer2_nick_name_textlar.id = trainer2_nick_namelar.value
	--
	join clublar(_op_mode, null) clublar on clublar.id = dancerlar.club 
	join textlar club_textlar on club_textlar.id = clublar.value
	--
	join citilar(_op_mode, null) citilar on citilar.id = clublar.citi 
	join textlar citi_textlar on citi_textlar.id = citilar.value
	--
	left join classlar st_classlar on st_classlar.id = dancerlar.st_class
	left join classlar la_classlar on la_classlar.id = dancerlar.la_class
	order by dancerlar.external_id
		, last_name_textlar.value
		, first_name_textlar.value
		, second_name_textlar.value
		, nick_name_textlar.value
$$ language sql;

-- select * from export_dancerlar(0::smallint); 
-- where external_id = 5590106;
-- select * from dancerlar d where external_id = 5590106;
-- select * from personlar p where id = 294;
--
-- select * from citilar(-1::smallint);

--
-- -- join first_namelar(-1::smallint, null) first_namelar on first_namelar.id = personlar.first_name
-- -- join textlar first_name_textlar on first_name_textlar.id = first_namelar.value
-- -- join clublar(-1::smallint, null) clublar on clublar.id = judgelar.club
-- -- join textlar club_textlar on club_textlar.id = clublar.value
-- -- join citilar(-1::smallint, null) citilar on citilar.id = clublar.citi
-- -- join textlar citi_textlar on citi_textlar.id = citilar.value
-- ;

-- ==================================================

--drop procedure determine_gender();
create or replace procedure determine_gender() as $$
begin
	update second_namelar o 
	set default_gender = (select id from genderlar where value = 'М')
	where o.default_gender is null and (false
		or (select value from textlar where id = o.value) like '%вич' 
		or (select value from textlar where id = o.value) like '% оглы' 
	);

	update second_namelar o 
	set default_gender = (select id from genderlar where value = 'Ж')
	where o.default_gender is null and (false
		or (select value from textlar where id = o.value) like '%вна' 
		or (select value from textlar where id = o.value) like '%чна'
	);

	update first_namelar o
	set default_gender = (
		select sn.default_gender
		from personlar p
		join first_namelar fn on fn.id = p.first_name
		join second_namelar sn on sn.id = p.second_name
		where o.id = fn.id and sn.default_gender is not null
		limit 1
	)
	where o.id in (
		select id from (
			select fn.id first_name, count(distinct sn.default_gender) qt
			from personlar p 
			join first_namelar fn on fn.id = p.first_name
			join second_namelar sn on sn.id = p.second_name
			group by fn.id
		) t
		where t.qt = 1
	) and default_gender is null;

	update first_namelar o
	set default_gender = (select id from genderlar where value = 'Ж')
	where default_gender is null and
		(select value from textlar where id = o.value) in ('Виктория-Софи', 'Стефания', 'Эмили')
	;

	update first_namelar o
	set default_gender = (select id from genderlar where value = 'М')
	where default_gender is null and
		(select value from textlar where id = o.value) in ('Даниэл-Алин', 'Олаф-Лорэнс', 'Захар', 'Даниэль') 
	;

	update personlar p
	set gender = (
		select fn.default_gender
		from first_namelar fn
		where fn.id = p.first_name
	)
	where p.id in (
		select p.id
		from personlar p 
		join first_namelar fn on fn.id = p.first_name
		join second_namelar sn on sn.id = p.second_name
		join textlar t on t.id = sn.value
		where fn.default_gender = sn.default_gender or fn.default_gender is not null and t.value = ''
	) and gender is null;
end
$$ language plpgsql;

-- call determine_gender();

-- select snt.value, fnt.value 
-- from personlar p 
-- join first_namelar fn on fn.id = p.first_name
-- join textlar fnt on fnt.id = fn.value
-- join last_namelar sn on sn.id = p.last_name
-- join textlar snt on snt.id = sn.value
-- where p.gender is null;

-- select get_init_data(0::smallint, null);


-- drop function if exists new_club(text, text, smallint);
-- create or replace function new_club(_club_value text, _citi_value text, _op_mode smallint) returns jsonb as $$
-- declare 
-- 	_jarr json[];
-- 	_ret jsonb = '{}';
-- 	_row record;
-- 	_club smallint;
-- 	_citi smallint;
-- begin
-- 	_club = add_club(_club_value, _citi_value, _op_mode);
-- 	select into _citi citi from clublar where id = _club;
--
-- 	_jarr = '{}';
-- 	for _row in
--         select id, value from clublar where id = _club
--     loop
-- 		_jarr = array_append(_jarr, row_to_json(_row));
--     end loop;
-- 	_ret = jsonb_set(_ret, '{clublar}', to_jsonb(_jarr));
--
-- 	_jarr = '{}';
-- 	for _row in
--         select id, value from citilar where id = _citi
--     loop
-- 		_jarr = array_append(_jarr, row_to_json(_row));
--     end loop;
-- 	_ret = jsonb_set(_ret, '{citilar}', to_jsonb(_jarr));
--
-- 	return _ret;
-- end;
-- $$ language plpgsql;

--select new_club('some', 'thing', 1::smallint);

--select id, value from citilar order by "name";

-- select person 
-- from dancerlar d 
-- join personlar p on p.id = d.person
-- join last_namelar ln2 on ln2.id = p.last_name
-- join first_namelar fn2 on fn2.id = p.first_name
-- where st_class in (8, 9, 10)
-- order by last_name, first_name
-- ;

-- select commit('{
--    "club": {
--      "id": null,
--      "value": "ClubSome"
--    }
--  }', 4::smallint);

drop function if exists "commit_club"(jsonb, smallint);
create or replace function "commit_club"(_item jsonb, _op_mode smallint) returns jsonb as $$
declare
	_ret jsonb;
	_is_none_all bool = true;
    _id smallint;
   	_id_found smallint;
    _value text;
    _citi smallint;
    _value_eta text;
    _citi_eta smallint;
begin
	_id = (_item->>'id')::smallint;
	_value = _item->>'value';
	_citi = (_item->>'citi')::smallint;
	if _id is null then
		if _value is null or _value = '' then
			return '{ 
				"err": {
                    "fields": {
                        "value": "Название клуба должно быть задано"
                    }
				}
			}';
		end if; 

		if _citi is null or _citi = 0 then
			return '{ 
				"err": {
					"fields": {
						"citi": "Город клуба должен быть задан"
					}
				}
			}';
		end if;

		if exists (select id from clublar where value = _value and citi = _citi and op_mode in (-1, _op_mode)) then
			return '{ 
				"err": {
					"fields": {
						"value": "Клуб с таким названием в этом городе уже существует",
						"citi": "Клуб с таким названием в этом городе уже существует"
					}
				}
			}';
		end if;
		insert into clublar (value, citi, op_mode)
		values (_value, _citi, _op_mode)
		returning id into _id;
		return format('{
			"ok": {
				"club": {
					"id": %s,
					"value": "%s",
					"citi": "%s"
				}
			}
		}', _id, _value, _citi);
	end if;

	select into _value_eta, _citi_eta value, citi from clublar where id = _id;
	if _value_eta is null then
		return format('{
			"err": {
				"message": "Клуб id''%s'' не найден",
				"payload": %s
			}
		}', _id, _item);
	end if;

	_ret = format('{ "id": %s }', _id);
	if not (_value is null or _value = '' or _value_eta = _value) then
		update clublar set value = _value where id = _id;
		_ret = jsonb_set(_ret, '{value}', to_jsonb(_value));
		_is_none_all = false;
	end if;
	if not (_citi is null or _citi = '' or _citi_eta = _citi) then
		update clublar set citi = _citi where id = _id;
		_ret = jsonb_set(_ret, '{citi}', to_jsonb(_citi));
		_is_none_all = false;
	end if;
	_ret = format('{ "club": %s }', _ret);
	if _is_none_all then
		_ret = jsonb_set(_ret, '{is_none_all}', to_jsonb(_is_none_all));
	end if;
	return format('{ "ok": %s }', _ret);
end;
$$ language plpgsql;

-- select commit('{
--    "citi": {
--      "id": null,
--      "value": "Thing2"
--    }
--  }', 4::smallint);
--
-- select commit('{
--    "citi": {
--      "id": 14,
--      "value": "Thing4"
--    }
--  }', 4::smallint);
--
-- select commit('{
--    "citi": {
--      "id": -1,
--      "value": "Thing3"
--    }
--  }', 4::smallint);
--
-- select commit('{
--    "citi": {
--      "id": null,
--      "value": ""
--    }
--  }', 4::smallint);
--
-- select commit('{
--    "citi": {
--      "id": null,
--      "value": null
--    }
--  }', 4::smallint);

drop function if exists "commit_citi"(jsonb, smallint);
create or replace function "commit_citi"(_item jsonb, _op_mode smallint) returns jsonb as $$
declare
	_ret jsonb;
	_is_none_all bool = true;
    _id smallint;
   	_id_found smallint;
    _value text;
    _value_eta text;
begin
	_id = (_item->>'id')::smallint;
	_value = _item->>'value';
	if _id is null then
		if _value is null or _value = '' then
			return '{ 
				"err": {
                    "fields": {
                        "value": "Название города должно быть задано"
                    }
				}
			}';
		end if;
		if exists (select id from citilar where value = _value and op_mode in (-1, _op_mode)) then
			return '{ 
				"err": {
					"fields": {
						"value": "Город с таким названием уже существует"
					}
				}
			}';
		end if;
		insert into citilar (value, op_mode)
		values (_value, _op_mode)
		returning id into _id;
		return format('{
			"ok": {
			   "citi": {
				 "id": %s,
				 "value": "%s"
			   }
			}
		}', _id, _value);
	end if;

	select into _value_eta value from citilar where id = _id;
	if _value_eta is null then
		return format('{
			"err": {
				"message": "Город id''%s'' не найден",
				"payload": %s
			}
		}', _id, _item);
	end if;

	_ret = format('{ "id": %s }', _id);
	if not (_value is null or _value = '' or _value_eta = _value) then
		update citilar set value = _value where id = _id;
		_ret = jsonb_set(_ret, '{value}', to_jsonb(_value));
		_is_none_all = false;
	end if;
	_ret = format('{ "citi": %s }', _ret);
	if _is_none_all then
		_ret = jsonb_set(_ret, '{is_none_all}', to_jsonb(_is_none_all));
	end if;
	return format('{ "ok": %s }', _ret);
end;
$$ language plpgsql;

drop function if exists "commit"(jsonb, smallint);
create or replace function "commit"(_modal jsonb, _op_mode smallint) returns jsonb as $$
declare
    _item jsonb;
    _id smallint;
   	_id_found smallint;
    _value text;
begin
    _item = _modal->'citi';
    if _item is not null then
    	return commit_citi(_item, _op_mode);
    end if;
    _item = _modal->'club';
    if _item is not null then
    	return commit_club(_item, _op_mode);
    end if;
    return format('{
		"err": {
			"message": "Неподдерживаемый тип modal",
			"payload": %s
		}
	}', _modal);
end;
$$ language plpgsql;

-- select commit('{
--    "citi2": {
--      "id": null,
--      "value": null
--    }
--  }', 4::smallint);



--Москва

