-- select 
--     d.external_id 
-- from dancerlar(-1::smallint, null::timestamptz) d
-- join personlar(-1::smallint, null::timestamptz) p on p.id = d.person
-- join first_namelar(-1::smallint, null::timestamptz) first_namelar on first_namelar.id = p.first_name
-- join textlar first_name_textlar on first_name_textlar.id = first_namelar.value
-- join last_namelar(-1::smallint, null::timestamptz) last_namelar on last_namelar.id = p.last_name
-- join textlar last_name_textlar on last_name_textlar.id = last_namelar.value
-- join clublar(-1::smallint, null::timestamptz) clublar on clublar.id = d.club
-- join citilar(-1::smallint, null::timestamptz) citilar on citilar.id = clublar.citi
-- join textlar club_textlar on club_textlar.id = clublar.value
-- join textlar citi_textlar on citi_textlar.id = citilar.value
-- where not d.is_archive
--     and first_name_textlar.value = 'Ульяна'
--     and last_name_textlar.value = 'Гашилова'
--     and club_textlar.value = 'Мечта'
--     and citi_textlar.value = 'Тула'
-- ;

-- delete from eventlar where id in (1, 2, 3);
-- delete from event_resultlar where "event" in (1, 2, 3);

drop function if exists add_event_result(smallint
    , integer
    , integer
    , integer
    , smallint
    , smallint
    , smallint
    , smallint
    , smallint
    , text
    , text
    , text
    , text);
CREATE OR REPLACE FUNCTION public.add_event_result(_op_mode smallint
    , _event integer
    , _category integer
    , _external_id integer
    , _couple_num smallint
    , _st_score smallint
    , _la_score smallint
    , _st_la_score smallint
    , _points smallint
    , _first_name text
    , _last_name text
    , _club text
    , _citi text
    , _dry_run boolean
)
 RETURNS integer
 LANGUAGE plpgsql
AS $function$
begin
    if _external_id is null then
        select 
            into _external_id 
            d.external_id 
        from dancerlar(_op_mode, null::timestamptz) d
        join personlar(_op_mode, null::timestamptz) p on p.id = d.person

        join first_namelar(_op_mode, null::timestamptz) first_namelar on first_namelar.id = p.first_name
        join textlar first_name_textlar on first_name_textlar.id = first_namelar.value
        join last_namelar(_op_mode, null::timestamptz) last_namelar on last_namelar.id = p.last_name
        join textlar last_name_textlar on last_name_textlar.id = last_namelar.value
        join clublar(_op_mode, null::timestamptz) clublar on clublar.id = d.club
        -- join citilar(_op_mode, null::timestamptz) citilar on citilar.id = clublar.citi
        join textlar club_textlar on club_textlar.id = clublar.value
        -- join textlar citi_textlar on citi_textlar.id = citilar.value
        where true
            and first_name_textlar.value = _first_name
            and last_name_textlar.value = _last_name
            and club_textlar.value = _club
            -- and citi_textlar.value = _citi
        ;
    end if;

    if _external_id is not null then
        if not _dry_run then
            insert into event_resultlar (op_mode, id
                , event
                , category
                , external_id
                , couple_num 
                -- , place
                , st_score 
                , la_score 
                , st_la_score 
                , points 
            ) 
            values (_op_mode, (select coalesce(max(id), 0) + 1 from event_resultlar)
                , _event
                , _category
                , _external_id
                , _couple_num 
                -- , _place
                , _st_score 
                , _la_score 
                , _st_la_score 
                , _points 
            )
            on conflict (id, created_at, op_mode)
            do update set event = excluded.event 
                , category = excluded.category
                , external_id = excluded.external_id
                , couple_num = excluded.couple_num
                -- , place = excluded.place
                , st_score = excluded.st_score 
                , la_score = excluded.la_score 
                , st_la_score = excluded.st_la_score 
                , points = excluded.points 
            -- returning id into _ret
            ;
        end if;
        return _external_id;
    else 
        return 0;
	end if;

end;
$function$
;
