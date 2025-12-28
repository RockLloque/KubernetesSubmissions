import { Pool } from 'pg';

const pool = new Pool({
  host: process.env.POSTGRES_HOST,
  port: parseInt(process.env.POSTGRES_PORT || '5432'),
  database: process.env.POSTGRES_DB,
  user: process.env.POSTGRES_USER,
  password: process.env.POSTGRES_PASSWORD,
  max: 20,
  idleTimeoutMillis: 30000,
  connectionTimeoutMillis: 2000,
});

pool.on('error', (err) => {
  console.error('Unexpected database error:', err);
});

export async function initializeDatabase(): Promise<void> {
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
  } catch (error) {
    console.error('Failed to initialize database:', error);
    throw error;
  } finally {
    client.release();
  }
}

export async function getCount(): Promise<number> {
  const client = await pool.connect();
  try {
    const result = await client.query(
      'SELECT count FROM ping_counter WHERE id = 1'
    );
    return result.rows[0]?.count || 0;
  } finally {
    client.release();
  }
}

export async function incrementCount(): Promise<number> {
  const client = await pool.connect();
  try {
    const result = await client.query(`
      UPDATE ping_counter
      SET count = count + 1, last_updated = CURRENT_TIMESTAMP
      WHERE id = 1
      RETURNING count
    `);
    return result.rows[0].count;
  } finally {
    client.release();
  }
}

export async function checkDatabaseHealth(): Promise<boolean> {
  try {
    const client = await pool.connect();
    await client.query('SELECT 1');
    client.release();
    return true;
  } catch (error) {
    console.error('Database health check failed:', error);
    return false;
  }
}

export async function closeDatabaseConnection(): Promise<void> {
  await pool.end();
}
