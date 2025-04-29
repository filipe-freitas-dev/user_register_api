-- First drop the triggers
DROP TRIGGER IF EXISTS update_users_modtime ON users;
DROP TRIGGER IF EXISTS update_admins_modtime ON admins;

-- Then drop the trigger function
DROP FUNCTION IF EXISTS update_modified_column();

-- Drop the tables in the correct order to respect foreign key constraints
DROP TABLE IF EXISTS admins;
DROP TABLE IF EXISTS users;

-- Finally, drop the extension if needed
-- Note: You might want to keep this commented out if other tables use this extension
-- DROP EXTENSION IF EXISTS "uuid-ossp";