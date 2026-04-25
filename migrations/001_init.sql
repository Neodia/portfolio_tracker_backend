CREATE TABLE assets (
    id SERIAL PRIMARY KEY ,
    ticker character varying(20) NOT NULL,
    chain character varying(50) NOT NULL,
    contract_address character varying(255) NOT NULL
);