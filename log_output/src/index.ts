import crypto from "crypto";
import express from 'express';
import dotenv from 'dotenv';

dotenv.config();

const app = express();

const randomHash = crypto.randomUUID();
const PORT = process.env.PORT ?? '3000';
const PING_PONG_PORT = process.env.PING_PONG_PORT ?? '3001';
const PING_PONG_SERVICE_NAME = process.env.PING_PONG_SERVICE_NAME ?? 'ping-pong-svc';

const randomHashFn = (hash: string) => {
  return `${new Date().toISOString()}: ${hash}`;
}



app.get('/', (_req, res) => {
  const count = 0; // TODO: read from PingPong
  return res.send(`${randomHashFn(randomHash)}\nPing / Pongs: ${count}`);
})

app.listen(PORT, () => console.log(`Listening on port: ${PORT}`))


