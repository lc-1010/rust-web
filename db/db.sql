drop table if exists course;

create table course 
(
    id serial primary key,
    teacher_id INT not null,
    name varchar(140) not null,
    time TIMESTAMP default now(),
    description varchar(140) not null,
    format varchar(140) not null, 
    structture  varchar(140) not null, 
    duration varchar(140) not null, 
    price INT not null,
    language varchar(140) not null, 
    level varchar(140) not null, 
);

insert into course 
    (  teacher_id, name,  description,
   format ,structture,duration,price, language, 
    level)
values( 1,'First course', 
 'description-jkdslfasljf ',
 'format-232323','structture=fjak',
 'duration-djsakl',
 1,
 'language-en',
    'level-A'
  );


insert into course 
    (  teacher_id, name,  description,
   format ,structture,duration,price, language, 
    level)
values( 1,'sencond course', 
 'description-jkdslfasljf ',
 'format-232323','structture=fjak',
 'duration-djsakl',
 1,
 'language-en',
    'level-A'
  );


/* 
ALTER TABLE public.course ALTER COLUMN description DROP NOT NULL;
ALTER TABLE public.course ALTER COLUMN format DROP NOT NULL;
ALTER TABLE public.course ALTER COLUMN structture DROP NOT NULL;
ALTER TABLE public.course ALTER COLUMN duration DROP NOT NULL;
ALTER TABLE public.course ALTER COLUMN price DROP NOT NULL;
ALTER TABLE public.course ALTER COLUMN "language" DROP NOT NULL;
ALTER TABLE public.course ALTER COLUMN "level" DROP NOT NULL;
*/


-- public.course definition

-- Drop table

-- DROP TABLE public.course;

CREATE TABLE public.course (
	id serial NOT NULL,
	teacher_id int4 NOT NULL,
	"name" varchar(140) NOT NULL,
	"time" timestamp NULL DEFAULT now(),
	description varchar(140) NULL,
	format varchar(140) NULL,
	structture varchar(140) NULL,
	duration varchar(140) NULL,
	price int4 NULL,
	"language" varchar(140) NULL,
	"level" varchar(140) NULL,
	CONSTRAINT course_pkey PRIMARY KEY (id)
);

/*
如果是not null 的 option<string> 需要进行 ！处理

ALTER TABLE public.course ALTER COLUMN description SET NOT NULL;

*/