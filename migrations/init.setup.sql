drop table if exists item_home_link;
drop table if exists item_type_home_link;
drop table if exists fridge_item_link;
drop table if exists item_fridge_link;
drop table if exists home_fridge_link;
drop table if exists user_home_link;
drop table if exists item_type;
drop table if exists item;
drop table if exists fridge;
drop table if exists home;
drop table if exists "user";

create table "user"
(
    id    int primary key,
    email varchar(255) not null,
    name  varchar(255) not null,
    constraint email_unique unique (email)
);

create table home
(
    id         int primary key generated always as identity,
    name       varchar(255) not null,
    created_by int          not null,
    created_at date         not null default current_timestamp,
    foreign key (created_by) references "user" (id)
);

create table user_home_link
(
    user_id   int,
    home_id   int,
    join_date date not null default current_timestamp,
    primary key (user_id, home_id),
    foreign key (user_id) references "user" (id),
    foreign key (home_id) references home (id)
);

create table fridge
(
    id         int primary key generated always as identity,
    name       varchar(255) not null,
    created_by int          not null,
    created_at date         not null default current_timestamp,
    foreign key (created_by) references "user" (id)
);

create table home_fridge_link
(
    home_id   int,
    fridge_id int,
    primary key (home_id, fridge_id),
    foreign key (home_id) references home (id),
    foreign key (fridge_id) references fridge (id)
);

create table item_type
(
    id         int primary key generated always as identity,
    name       varchar(255) not null,
    created_by int          not null,
    created_at date         not null default current_timestamp,
    foreign key (created_by) references "user" (id)
);

create table item
(
    id           int primary key generated always as identity,
    name         varchar(255) not null,
    item_type_id int          not null,
    created_by   int          not null,
    created_at   date         not null default current_timestamp,
    foreign key (created_by) references "user" (id),
    foreign key (item_type_id) references item_type (id)
);

create table item_type_home_link
(
    item_type_id int,
    home_id      int,
    primary key (item_type_id, home_id),
    foreign key (item_type_id) references item_type (id),
    foreign key (home_id) references home (id)
);

create table item_fridge_link
(
    id              int primary key generated always as identity,
    item_id         int,
    fridge_id       int,
    expiration_date date not null,
    foreign key (item_id) references item (id),
    foreign key (fridge_id) references fridge (id)
);

-- Insert mock data into user table
INSERT INTO "user" (id, email, name)
VALUES (1, 'john.doe@example.com', 'John Doe'),
       (2, 'jane.smith@example.com', 'Jane Smith');

-- Insert mock data into home table
INSERT INTO home (name, created_by)
VALUES ('Home 1', 1),
       ('Home 2', 2);

-- Insert mock data into user_home_link table
INSERT INTO user_home_link (user_id, home_id, join_date)
VALUES (1, 1, '2023-01-01'),
       (2, 2, '2023-01-02');

-- Insert mock data into fridge table
INSERT INTO fridge (name, created_by)
VALUES ('Fridge 1', 1),
       ('Fridge 2', 2);

-- Insert mock data into home_fridge_link table
INSERT INTO home_fridge_link (home_id, fridge_id)
VALUES (1, 1),
       (2, 2);

-- Insert mock data into item_type table
INSERT INTO item_type (name, created_by)
VALUES ('Dairy', 1),
       ('Vegetables', 2);

-- Insert mock data into item table
INSERT INTO item (name, item_type_id, created_by)
VALUES ('Milk', 1, 1),
       ('Carrot', 2, 2);

-- Insert mock data into item_type_home_link table
INSERT INTO item_type_home_link (item_type_id, home_id)
VALUES (1, 1),
       (2, 2);

-- Insert mock data into fridge_item_link table
INSERT INTO item_fridge_link (item_id, fridge_id, expiration_date)
VALUES (1, 1, '2023-12-01'),
       (2, 2, '2023-12-02');
