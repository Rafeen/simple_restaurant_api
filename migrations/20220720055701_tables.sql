-- Add migration script here
CREATE TABLE IF NOT EXISTS items
(
    id        serial primary key,
    name      varchar(255) not null,
    duration  float        not null default 0,
    price     float        not null default 0,
    available boolean      not null default true
);

CREATE TABLE IF NOT EXISTS tables
(
    id serial primary key
);

CREATE TABLE IF NOT EXISTS order_items
(
    table_id  int  not null,
    item_id   int  not null,
    quantity int  not null default 0,
    served   bool not null default false,
    timestamp timestamp default current_timestamp,
    serving_at timestamp,
    PRIMARY KEY (table_id, item_id)
);

alter table order_items
    add foreign key (table_id) REFERENCES "tables" ("id") ON DELETE CASCADE ON UPDATE CASCADE ;
alter table order_items
    add foreign key (item_id) REFERENCES "items" ("id") ON DELETE CASCADE ON UPDATE CASCADE ;
