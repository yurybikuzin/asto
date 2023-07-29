
-- ==================================================

drop table if exists eventlar cascade; 
create table eventlar (
	op_mode smallint not null references op_modelar(id) on delete cascade
	, created_at timestamptz not null default now()
	, id int not null
    , "date" date not null
	, title int not null references textlar(id) on delete cascade
	, unique(id, created_at, op_mode)
);
drop function if exists eventlar(smallint
    , timestamptz
);
create or replace function eventlar(_op_mode smallint
    , _created_before_or_at timestamptz
) 
returns table (id int
    , "date" date
	, title int
)
as $$
	select distinct on(id)
		id, "date", title
	from eventlar
	where op_mode in (-1, _op_mode) 
		and (_created_before_or_at is null or created_at <= _created_before_or_at)
	order by id, created_at desc, op_mode
$$ language sql;
drop function if exists add_event(smallint
    , int
    , date
    , text
);
create or replace function add_event(_op_mode smallint
    , _id int
    , _date date
    , _title_value text)

returns int as $$
declare 
	_ret int;
    _title int;
    _need_add bool = false;
begin
	_title = add_text(_title_value);

	if _id is null then
		select 
			into _id
			id
		from eventlar(_op_mode, null) 
		where true 
            and "date" = _date 
            and title = _title 
		limit 1;
	end if;

	if _id is null then
		_need_add = true;
	else 
		if not exists (
			select id
            from eventlar(_op_mode, null) 
			where id = _id
                and "date" = _date 
                and title = _title 
			limit 1
		) then
			_need_add = true;
		end if;
	end if;

	if not _need_add then
		_ret = _id;
	else
		insert into eventlar (op_mode, id, "date", title) 
		values (_op_mode, coalesce(_id, (select coalesce(max(id), 0) + 1 from eventlar))
            , _date
			, _title
		)
		on conflict (id, created_at, op_mode)
		do update set op_mode = excluded.op_mode
			,  "date" = excluded."date"
			,  title = excluded.title
		returning id into _ret;
	end if;

	return _ret;
end;
$$ language plpgsql;

-- ==================================================

