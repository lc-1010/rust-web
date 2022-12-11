drop table if exists course;

create table course 
(
    id serial primary key,
    teacher_id INT not null,
    name varchar(140) not null,
    time TIMESTAMP default now()
);

insert into course 
    (id, teacher_id, name, time )
values(1,1,'First course', '2022-12-17 05:11:11');
insert into course
    (id, teacher_id, name, time )
values(2,1, 'Sencond course', '2022-12-11 01:11:11');
