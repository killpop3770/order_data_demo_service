--DROP TABLE IF EXIST orders;
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    order_uid character varying(255) NOT NULL UNIQUE,
    body JSONB NOT NULL
);