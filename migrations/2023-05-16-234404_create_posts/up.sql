create table sprintnum_date
(
    sprint_num  int  not null,
    sprint_date date not null,
    /*primary key (sprint_num, sprint_date)*/
);
create table team_report
(
    teams              varchar(50)  not null,
    sprint_num         int not null,
    understand_easiest varchar(300) null,
    understand_hardest varchar(300) null,
    approach_easiest   varchar(300) null,
    approach_hardest   varchar(300) null,
    solve_easiest      varchar(300) null,
    solve_hardest      varchar(300) null,
    evaluate_easiest   varchar(300) null,
    evaluate_hardest   varchar(300) null,
    completion         float null,
    contact            varchar(300) null,
    comments           varchar(300) null,
    /*primary key (teams, sprint_num)*/
    /*constraint team_report_ibfk_1
        foreign key (sprint_num) references sprintnum_date (sprint_num)*/
);


