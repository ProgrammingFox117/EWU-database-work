create table sprint_num_dates
(
    sprint_num  int  not null,
    sprint_date date not null,
    primary key (sprint_num)
);
create table team_reports
(
    teams              varchar(50) not null ,
    sprint_num         int not null,
    understand_easiest varchar(300) null,
    understand_hardest varchar(300) null,
    approach_easiest   varchar(300) null,
    approach_hardest   varchar(300) null,
    solve_easiest      varchar(300) null,
    solve_hardest      varchar(300) null,
    evaluate_easiest   varchar(300) null,
    evaluate_hardest   varchar(300) null,
    completion         int null,
    contact            varchar(300) null,
    comments           varchar(300) null,
    primary key (teams, sprint_num)
    /*constraint team_report_ibfk_1
        foreign key (sprint_num) references sprintnum_date (sprint_num)*/
);



create table individual_reports
(
    ouath_id           varchar(255) not null,
    sprint_num         int          not null,
    monday_time        int          null,
    tuesday_time       int          null,
    wednesday_time     int          null,
    thursday_time      int          null,
    friday_time        int          null,
    saturday_time      int          null,
    sunday_time        int          null,
    discrepancy        varchar(300) null,
    request            varchar(300) null,
    primary key (ouath_id,sprint_num)

  /*  constraint individual_report_ibfk_1
        foreign key (sprint_num) references sprintnum_date (sprint_num)
*/
);