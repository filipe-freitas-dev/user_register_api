DROP TRIGGER IF EXISTS update_users_modtime ON users;
DROP TRIGGER IF EXISTS update_admins_modtime ON admins;

DROP FUNCTION IF EXISTS update_modified_column();

DROP TABLE IF EXISTS admins;
DROP TABLE IF EXISTS users;

DROP EXTENSION IF EXISTS "uuid-ossp";