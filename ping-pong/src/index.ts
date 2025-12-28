import express from 'express';
import dotenv from 'dotenv';
import {
  initializeDatabase,
  getCount,
  incrementCount,
  checkDatabaseHealth,
  closeDatabaseConnection
} from './db/client';

dotenv.config();

const app = express();
const PORT = process.env.PORT ?? 3001;

app.get("/pingpong", async (_req, res) => {
  try {
    const count = await incrementCount();
    res.send(`PingPong counter: ${count}`);
  } catch (error) {
    console.error('Error incrementing counter:', error);
    res.status(500).send('Failed to increment counter');
  }
});

app.get("/pings", async (_req, res) => {
  try {
    const count = await getCount();
    res.send(count.toString());
  } catch (error) {
    console.error('Error getting counter:', error);
    res.status(500).send('Failed to get counter');
  }
});

app.get("/health", async (_req, res) => {
  const dbHealthy = await checkDatabaseHealth();

  if (dbHealthy) {
    res.status(200).json({ status: 'healthy', database: 'connected' });
  } else {
    res.status(503).json({ status: 'unhealthy', database: 'disconnected' });
  }
});

async function startServer() {
  try {
    console.log('Initializing database...');
    await initializeDatabase();
    console.log('Database ready');

    app.listen(PORT, () => {
      console.log(`PingPong is listening on port: ${PORT}`);
    });
  } catch (error) {
    console.error('Failed to start server:', error);
    process.exit(1);
  }
}

process.on('SIGTERM', async () => {
  console.log('SIGTERM received, closing database connection...');
  await closeDatabaseConnection();
  process.exit(0);
});

process.on('SIGINT', async () => {
  console.log('SIGINT received, closing database connection...');
  await closeDatabaseConnection();
  process.exit(0);
});

startServer();
