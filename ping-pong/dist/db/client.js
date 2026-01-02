"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.initializeDatabase = initializeDatabase;
exports.getCount = getCount;
exports.incrementCount = incrementCount;
exports.checkDatabaseHealth = checkDatabaseHealth;
exports.closeDatabaseConnection = closeDatabaseConnection;
const pg_1 = require("pg");
const env_1 = require("../env");
const pool = new pg_1.Pool({
    host: env_1.env.POSTGRES_HOST,
    port: env_1.env.POSTGRES_PORT,
    database: env_1.env.POSTGRES_DB,
    user: env_1.env.POSTGRES_USER,
    password: env_1.env.POSTGRES_PASSWORD,
    max: 20,
    idleTimeoutMillis: 30000,
    connectionTimeoutMillis: 2000,
});
pool.on('error', (err) => {
    console.error('Unexpected database error:', err);
});
async function initializeDatabase() {
    const client = await pool.connect();
    try {
        await client.query(`
      CREATE TABLE IF NOT EXISTS ping_counter (
        id INTEGER PRIMARY KEY DEFAULT 1,
        count INTEGER NOT NULL DEFAULT 0,
        last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
      );
    `);
        await client.query(`
      INSERT INTO ping_counter (id, count)
      VALUES (1, 0)
      ON CONFLICT (id) DO NOTHING;
    `);
        console.log('Database initialized successfully');
    }
    catch (error) {
        console.error('Failed to initialize database:', error);
        throw error;
    }
    finally {
        client.release();
    }
}
async function getCount() {
    const client = await pool.connect();
    try {
        const result = await client.query('SELECT count FROM ping_counter WHERE id = 1');
        return result.rows[0]?.count || 0;
    }
    finally {
        client.release();
    }
}
async function incrementCount() {
    const client = await pool.connect();
    try {
        const result = await client.query(`
      UPDATE ping_counter
      SET count = count + 1, last_updated = CURRENT_TIMESTAMP
      WHERE id = 1
      RETURNING count
    `);
        return result.rows[0].count;
    }
    finally {
        client.release();
    }
}
async function checkDatabaseHealth() {
    try {
        const client = await pool.connect();
        await client.query('SELECT 1');
        client.release();
        return true;
    }
    catch (error) {
        console.error('Database health check failed:', error);
        return false;
    }
}
async function closeDatabaseConnection() {
    await pool.end();
}
//# sourceMappingURL=client.js.map