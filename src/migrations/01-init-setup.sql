create table "user" (
    id int primary key,
    email varchar(255) not null,
    name varchar(255) not null
);

create table home (
    id int primary key generated always as identity,
    name varchar(255) not null
);

create table user_home_link (
    user_id int,
    home_id int,
    join_date date not null default current_timestamp,
    is_owner boolean not null default false,
    primary key (user_id, home_id),
    foreign key (user_id) references "user"(id),
    foreign key (home_id) references home(id)
);

create table fridge (
    id int primary key generated always as identity,
    name varchar(255) not null
);

create table home_fridge_link (
    home_id int,
    fridge_id int,
    primary key (home_id, fridge_id),
    foreign key (home_id) references home(id)
);

create table item_type (
    id int primary key generated always as identity,
    name varchar(255) not null
);

create table item (
    id int primary key generated always as identity,
    name varchar(255) not null,
    item_type_id int not null,
    expiration_date date not null,
    foreign key (item_type_id) references item_type(id)
);

create table item_type_home_link (
    item_type_id int,
    home_id int,
    primary key (item_type_id, home_id),
    foreign key (item_type_id) references item_type(id),
    foreign key (home_id) references home(id)
);

create table item_home_link (
    item_id int,
    home_id int,
    primary key (item_id, home_id),
    foreign key (item_id) references item(id),
    foreign key (home_id) references home(id)
);

create table fridge_item_link (
    item_id int,
    fridge_id int,
    primary key (item_id, fridge_id),
    foreign key (item_id) references item(id),
    foreign key (fridge_id) references fridge(id)
);

-- Insert mock data into user table
INSERT INTO "user" (id, email, name) VALUES
    (1, 'john.doe@example.com', 'John Doe'),
    (2, 'jane.smith@example.com', 'Jane Smith');

-- Insert mock data into home table
INSERT INTO home (name) VALUES
    ('Home 1'),
    ('Home 2');

-- Insert mock data into user_home_link table
INSERT INTO user_home_link (user_id, home_id, join_date, is_owner) VALUES
    (1, 1, '2023-01-01', true),
    (2, 2, '2023-01-02', false);

-- Insert mock data into fridge table
INSERT INTO fridge (name) VALUES
    ('Fridge 1'),
    ('Fridge 2');

-- Insert mock data into home_fridge_link table
INSERT INTO home_fridge_link (home_id, fridge_id) VALUES
    (1, 1),
    (2, 2);

-- Insert mock data into item_type table
INSERT INTO item_type (name) VALUES
                                 ('Dairy'),
                                 ('Vegetables');

-- Insert mock data into item table
INSERT INTO item (name, item_type_id, expiration_date) VALUES
    ('Milk', 1, '2023-12-01'),
    ('Carrot', 2, '2023-12-10');

-- Insert mock data into item_type_home_link table
INSERT INTO item_type_home_link (item_type_id, home_id) VALUES
    (1, 1),
    (2, 2);

-- Insert mock data into item_home_link table
INSERT INTO item_home_link (item_id, home_id) VALUES
    (1, 1),
    (2, 2);

-- Insert mock data into fridge_item_link table
INSERT INTO fridge_item_link (item_id, fridge_id) VALUES
    (1, 1),
    (2, 2);
