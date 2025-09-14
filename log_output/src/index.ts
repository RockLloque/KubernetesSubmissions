import crypto from "crypto";
import express from 'express';
import dotenv from 'dotenv';

dotenv.config();

const app = express();

const randomHash = crypto.randomUUID();
const PORT = process.env.PORT ?? '3000';
const PING_PONG_PORT = process.env.PING_PONG_PORT ?? '3001';
const PING_PONG_SERVICE_NAME = process.env.PING_PONG_SERVICE_NAME ?? 'ping-pong-svc';
const PING_PONG_SUBDIRECTORY = process.env.PING_PONG_SUBDIRECTORY ?? 'pings';
const PING_PONG_PATH = `http://${PING_PONG_SERVICE_NAME}:${PING_PONG_PORT}/${PING_PONG_SUBDIRECTORY}`;

console.log(`path: ${PING_PONG_PATH}`);

const randomHashFn = (hash: string) => {
  return `${new Date().toISOString()}: ${hash}`;
}



app.get('/', async (_req, res) => {
  try {
    const resp = await fetch(PING_PONG_PATH);

    if (!resp.ok) {
      throw new Error(`Ping Pong service under ${PING_PONG_PATH} returned the error: ${resp.status} `);
    }

    const count = await resp.text() ?? 0;

    return res.send(`${randomHashFn(randomHash)}\nPing / Pongs: ${count}`);
  } catch (error: any) {
    console.error(error.message);
    return res
      .status(500)
      .send(`${randomHashFn(randomHash)}\nPing / Pongs: Error`);
  }
})

app.listen(PORT, () => console.log(`Listening on port: ${PORT}`))


