create table products (
    id varchar(255) primary key,
    name varchar(255) not null,
    description text not null,
    created_at timestamptz not null,
    updated_at timestamptz not null
)
