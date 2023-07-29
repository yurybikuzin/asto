-- ==================================================

drop table if exists event_resultlar cascade;
create table event_resultlar ( 
	op_mode smallint not null references op_modelar(id) on delete cascade
	, created_at timestamptz not null default now()
	, id int not null
    , event int not null
	, category int not null references textlar(id) on delete cascade
	, external_id int not null
    , couple_num smallint not null
    -- , place smallint
    , st_score smallint
    , la_score smallint
    , st_la_score smallint
    , points smallint
	, unique(id, created_at, op_mode)
);
drop function if exists event_resultlar(smallint
    , timestamptz
);
create or replace function event_resultlar(_op_mode smallint
    , _created_before_or_at timestamptz
) 
returns table ( id int
    , event int 
	, category int 
	, external_id int 
    , couple_num smallint 
    -- , place smallint
    , st_score smallint
    , la_score smallint
    , st_la_score smallint
    , points smallint
)
as $$
	select distinct on(id) id
    , event 
	, category 
	, external_id 
    , couple_num 
    -- , place
    , st_score 
    , la_score 
    , st_la_score 
    , points 
	from event_resultlar
	where op_mode in (-1, _op_mode)
		and (_created_before_or_at is null or created_at <= _created_before_or_at)
	order by id, created_at desc, op_mode desc
$$ language sql;
-- drop function if exists add_event_result(smallint, int
--     , int
--     , int
-- 	, int
--     , smallint
--     , smallint
--     , smallint
--     , smallint
--     , smallint
--     , smallint
-- );
-- create or replace function add_event_result(_op_mode smallint, _id int
--     , _event int
--     , _category int
-- 	, _external_id int
--     , _couple_num smallint
--     , _place smallint
--     , _st_score smallint
--     , _la_score smallint
--     , _st_la_score smallint
--     , _points smallint
-- ) returns int as $$
-- declare 
-- 	_ret int;
--     _need_add bool = false;
-- begin
-- 	if _id is null then
-- 		select 
-- 			into _id
-- 			id
-- 		from event_resultlar(_op_mode, null) 
-- 		where true 
--             and event = _event 
--             and category = _category
--             and external_id = _external_id
-- 		limit 1;
-- 	end if;
--
-- 	if _id is null then
-- 		_need_add = true;
-- 	else 
-- 		if not exists (
-- 			select id
--             from event_resultlar(_op_mode, null) 
-- 			where id = _id
--                 and event = _event 
--                 and category = _category
--                 and external_id = _external_id
--                 and couple_num = _couple_num
--                 and place = _place
--                 and st_score = _st_score 
--                 and la_score = _la_score 
--                 and st_la_score = _st_la_score 
--                 and points = _points 
--             limit 1
-- 		) then
-- 			_need_add = true;
-- 		end if;
-- 	end if;
--
-- 	if not _need_add then
-- 		_ret = _id;
-- 	else
-- 		insert into event_resultlar (op_mode, id
-- 			, event
--             , category
-- 			, external_id
--             , couple_num 
--             , place
--             , st_score 
--             , la_score 
--             , st_la_score 
--             , points 
-- 		) 
-- 		values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from event_resultlar))
-- 			, _event
--             , _category
-- 			, _external_id
--             , _couple_num 
--             , _place
--             , _st_score 
--             , _la_score 
--             , _st_la_score 
--             , _points 
-- 		)
-- 		on conflict (id, created_at, op_mode)
-- 		do update set event = excluded.event 
--             , category = excluded.category
--             , external_id = excluded.external_id
--             , couple_num = excluded.couple_num
--             , place = excluded.place
--             , st_score = excluded.st_score 
--             , la_score = excluded.la_score 
--             , st_la_score = excluded.st_la_score 
--             , points = excluded.points 
-- 		returning id into _ret;
-- 	end if;
--
-- 	return _ret;
-- end;
-- $$ language plpgsql;

-- ==================================================

