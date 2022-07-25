-- Add migration script here
INSERT INTO items (id, name, duration, price, available)
VALUES (default, 'Tempura', 15, 220, true),
       (default, 'Sushi', 5, 110, true),
       (default, 'Kara age', 10, 150, true),
       (default, 'Sashimi', 8, 180, true),
       (default, 'Miso Soup', 5, 90, true);


INSERT INTO tables (id)
VALUES (default),
       (default),
       (default),
       (default),
       (default),
       (default),
       (default),
       (default),
       (default),
       (default),
       (default),
       (default),
       (default);