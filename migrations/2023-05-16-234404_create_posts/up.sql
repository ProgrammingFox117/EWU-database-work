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

/*create table requirements
(
    class       varchar(255) null,
    teams       varchar(255) not null,
    indexs      int auto_increment,
    description varchar(255) null,
    primary key (indexs, teams)

);*/

create table individual_report
(
    ouath_id           varchar(255) not null primary key,
    sprint_num             int          null,
    monday_time        int          null,
    tuesday_time       int          null,
    wednesday_time     int          null,
    thursday_time      int          null,
    friday_time        int          null,
    saturday_time      int          null,
    sunday_time        int          null,
    discrepancy        varchar(300) null,
    request            varchar(300) null,

  /*  constraint individual_report_ibfk_1
        foreign key (sprint_num) references sprintnum_date (sprint_num)
*/
);

/*create table team_activities
(
    teams          varchar(50)  null,
    ouath_id       varchar(255) not null primary key,
    sprint_num     int          null,
    activity_index int          null,
    answers        varchar(255) null,
 /*   constraint team_activities_ibfk_1
        foreign key (teams) references team_report (teams),
    constraint team_activities_ibfk_2
        foreign key (sprint_num) references sprintnum_date (sprint_num)*/
);*/

/*create table login
(
    ouath_id   varchar(255) not null
        primary key,
    is_teacher tinyint(1)   null,
    is_student tinyint(1)   null,
    teams      varchar(255) null,
    class      varchar(255) null,
    is_Admin   tinyint(1)   null,
/*    constraint login_ibfk_1
        foreign key (ouath_id) references individual_report (ouath_id),
    constraint login_ibfk_2
        foreign key (ouath_id) references team_activities (ouath_id)*/
);*/