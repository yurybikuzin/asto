
drop function if exists export_dancerlar_for_anton(smallint);
create or replace function export_dancerlar_for_anton(_op_mode smallint) 
returns table (
	external_id int
	, name text
	, second_name text
	, birth_date date
	, st_class text
	, la_class text
	, club text
	, citi text
	, chief text
	, trainer text
	, trainer2 text
	, region int
	, gender text
) as $$
	select dancerlar.external_id
		, last_name_textlar.value || ' ' || first_name_textlar.value name
		, second_name_textlar.value second_name
		, personlar.birth_date
		, st_classlar.value st_class
		, la_classlar.value la_class
		, club_textlar.value club
		, citi_textlar.value citi
        , case when clublar.chief is null then chief_last_name_textlar.value || ' ' || chief_first_name_textlar.value else null end as chief 
		, trainer_last_name_textlar.value || ' ' || trainer_first_name_textlar.value as trainer
        , case when dancerlar.trainer2 is null then trainer2_last_name_textlar.value || ' ' || trainer2_first_name_textlar.value else null end as trainer2 
		, citilar.region region
		, case 
			when personlar.gender = 1 then 'М'
			when personlar.gender = 2 then 'Ж'
			else null
		end gender
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
	--
	join clublar(_op_mode, null) clublar on clublar.id = dancerlar.club 
	left join personlar(_op_mode, null) chief_personlar on chief_personlar.id = clublar.chief 
	left join last_namelar(_op_mode, null) chief_last_namelar on chief_last_namelar.id = chief_personlar.last_name
	left join textlar chief_last_name_textlar on chief_last_name_textlar.id = chief_last_namelar.value
	left join first_namelar(_op_mode, null) chief_first_namelar on chief_first_namelar.id = chief_personlar.first_name
	left join textlar chief_first_name_textlar on chief_first_name_textlar.id = chief_first_namelar.value
	left join second_namelar(_op_mode, null) chief_second_namelar on chief_second_namelar.id = chief_personlar.second_name
	left join textlar chief_second_name_textlar on chief_second_name_textlar.id = chief_second_namelar.value
	--
	join trainerlar(_op_mode, null) trainerlar on trainerlar.id = dancerlar.trainer 
	join personlar(_op_mode, null) trainer_personlar on trainer_personlar.id = trainerlar.person 
	join last_namelar(_op_mode, null) trainer_last_namelar on trainer_last_namelar.id = trainer_personlar.last_name
	join textlar trainer_last_name_textlar on trainer_last_name_textlar.id = trainer_last_namelar.value
	join first_namelar(_op_mode, null) trainer_first_namelar on trainer_first_namelar.id = trainer_personlar.first_name
	join textlar trainer_first_name_textlar on trainer_first_name_textlar.id = trainer_first_namelar.value
	join second_namelar(_op_mode, null) trainer_second_namelar on trainer_second_namelar.id = trainer_personlar.second_name
	join textlar trainer_second_name_textlar on trainer_second_name_textlar.id = trainer_second_namelar.value
	--
	left join trainerlar(_op_mode, null) trainerlar2 on trainerlar.id = dancerlar.trainer2 
	left join personlar(_op_mode, null) trainer2_personlar on trainer2_personlar.id = trainerlar2.person 
	left join last_namelar(_op_mode, null) trainer2_last_namelar on trainer2_last_namelar.id = trainer2_personlar.last_name
	left join textlar trainer2_last_name_textlar on trainer2_last_name_textlar.id = trainer2_last_namelar.value
	left join first_namelar(_op_mode, null) trainer2_first_namelar on trainer2_first_namelar.id = trainer2_personlar.first_name
	left join textlar trainer2_first_name_textlar on trainer2_first_name_textlar.id = trainer2_first_namelar.value
	left join second_namelar(_op_mode, null) trainer2_second_namelar on trainer2_second_namelar.id = trainer2_personlar.second_name
	left join textlar trainer2_second_name_textlar on trainer2_second_name_textlar.id = trainer2_second_namelar.value
	--
	join textlar club_textlar on club_textlar.id = clublar.value
	--
	join citilar(_op_mode, null) citilar on citilar.id = clublar.citi 
	join textlar citi_textlar on citi_textlar.id = citilar.value
	--
	left join classlar st_classlar on st_classlar.id = dancerlar.st_class
	left join classlar la_classlar on la_classlar.id = dancerlar.la_class
	order by last_name_textlar.value
		, first_name_textlar.value
		, second_name_textlar.value
$$ language sql;

select * from export_dancerlar_for_anton(0::smallint);
