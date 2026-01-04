-- Create UUID extension if not exists
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create todos table
CREATE TABLE todos (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    sorting_order INTEGER NOT NULL DEFAULT 0,
    CONSTRAINT order_positive CHECK ("order" >= 0)
);

-- Create indexes for common queries
CREATE INDEX idx_todos_order ON todos("order");
