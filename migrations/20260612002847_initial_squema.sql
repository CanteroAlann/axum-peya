CREATE EXTENSION IF NOT EXISTS postgis;


CREATE TABLE restaurants (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    ubication GEOGRAPHY(Point, 4326) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE deliveries (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    position GEOGRAPHY(Point, 4326) NOT NULL,
    is_available BOOLEAN DEFAULT TRUE
);


CREATE INDEX idx_restaurants_ubication ON restaurants USING GIST(ubication);
CREATE INDEX idx_deliveries_position ON deliveries USING GIST(position);