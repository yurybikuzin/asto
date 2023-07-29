
drop function if exists get_init_data(smallint, timestamptz);
create or replace function get_init_data(_op_mode smallint, _created_before_or_at timestamptz) returns jsonb as $$
declare 
	_jarr json[];
	_ret jsonb = '{}';
	_row record;
begin
	_jarr = '{}';
	for _row in
        select id, value from textlar
    loop
		_jarr = array_append(_jarr, row_to_json(_row));
    end loop;
	_ret = jsonb_set(_ret, '{textlar}', to_jsonb(_jarr));

	_jarr = '{}';
	for _row in
        select id, value from classlar
    loop
		_jarr = array_append(_jarr, row_to_json(_row));
    end loop;
	_ret = jsonb_set(_ret, '{classlar}', to_jsonb(_jarr));

	_jarr = '{}';
	for _row in
        select id, value from categorilar 
    loop
		_jarr = array_append(_jarr, row_to_json(_row));
    end loop;
	_ret = jsonb_set(_ret, '{categorilar}', to_jsonb(_jarr));

	_jarr = '{}';
	for _row in
        select id, value from genderlar 
    loop
		_jarr = array_append(_jarr, row_to_json(_row));
    end loop;
	_ret = jsonb_set(_ret, '{genderlar}', to_jsonb(_jarr));

	_jarr = '{}';
	for _row in
        select id, value from last_namelar(_op_mode, _created_before_or_at)
    loop
		_jarr = array_append(_jarr, row_to_json(_row));
    end loop;
	_ret = jsonb_set(_ret, '{last_namelar}', to_jsonb(_jarr));

	_jarr = '{}';
	for _row in
        select id, value, default_gender from first_namelar(_op_mode, _created_before_or_at)
    loop
		_jarr = array_append(_jarr, row_to_json(_row));
    end loop;
	_ret = jsonb_set(_ret, '{first_namelar}', to_jsonb(_jarr));

	_jarr = '{}';
	for _row in
        select id, value, default_gender from second_namelar(_op_mode, _created_before_or_at) 
    loop
		_jarr = array_append(_jarr, row_to_json(_row));
    end loop;
	_ret = jsonb_set(_ret, '{second_namelar}', to_jsonb(_jarr));

	_jarr = '{}';
	for _row in
        select id, value from nick_namelar(_op_mode, _created_before_or_at)
    loop
		_jarr = array_append(_jarr, row_to_json(_row));
    end loop;
	_ret = jsonb_set(_ret, '{nick_namelar}', to_jsonb(_jarr));

	_jarr = '{}';
	for _row in
        select id, value from citilar(_op_mode, _created_before_or_at)
    loop
		_jarr = array_append(_jarr, row_to_json(_row));
    end loop;
	_ret = jsonb_set(_ret, '{citilar}', to_jsonb(_jarr));

	_jarr = '{}';
	for _row in
        select id
			, value
			, citi 
			, chief
		from clublar(_op_mode, _created_before_or_at)
    loop
		_jarr = array_append(_jarr, row_to_json(_row));
    end loop;
	_ret = jsonb_set(_ret, '{clublar}', to_jsonb(_jarr));

	_jarr = '{}';
	for _row in
        select id
			, last_name
			, first_name
			, second_name
			, birth_date
			, nick_name
			, gender 
        from personlar(_op_mode, _created_before_or_at)
    loop
		_jarr = array_append(_jarr, row_to_json(_row));
    end loop;
	_ret = jsonb_set(_ret, '{personlar}', to_jsonb(_jarr));

	_jarr = '{}';
	for _row in
        select id
			, external_id
			, person
			, categori
			, assignment_date
			, club
			, number_of_participation_in_festivals
			, is_archive
		from judgelar(_op_mode, _created_before_or_at)
    loop
		_jarr = array_append(_jarr, row_to_json(_row));
    end loop;
	_ret = jsonb_set(_ret, '{judgelar}', to_jsonb(_jarr));

	_jarr = '{}';
	for _row in
        select id
			, person
			, club
        from trainerlar(_op_mode, _created_before_or_at)
    loop
		_jarr = array_append(_jarr, row_to_json(_row));
    end loop;
	_ret = jsonb_set(_ret, '{trainerlar}', to_jsonb(_jarr));

	_jarr = '{}';
	for _row in
        select id
			, external_id
			, person
			, st_class
			, la_class
			, st_score
			, la_score
			, st_la_score
			, points
			, club
			, trainer
			, trainer2
			, is_archive
		from dancerlar(_op_mode, _created_before_or_at)
    loop
		_jarr = array_append(_jarr, row_to_json(_row));
    end loop;
	_ret = jsonb_set(_ret, '{dancerlar}', to_jsonb(_jarr));

	_jarr = '{}';
	for _row in
        select id
			, date
            , title
		from eventlar(_op_mode, _created_before_or_at)
    loop
		_jarr = array_append(_jarr, row_to_json(_row));
    end loop;
	_ret = jsonb_set(_ret, '{eventlar}', to_jsonb(_jarr));

	_jarr = '{}';
	for _row in
        select id
            , event 
            , category 
            , external_id 
            , couple_num 
            -- , place 
            , st_score 
            , la_score 
            , st_la_score 
            , points 
		from event_resultlar(_op_mode, _created_before_or_at)
    loop
		_jarr = array_append(_jarr, row_to_json(_row));
    end loop;
	_ret = jsonb_set(_ret, '{event_resultlar}', to_jsonb(_jarr));

    return _ret;
end
$$ language plpgsql;

