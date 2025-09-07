import { readFileSync } from "fs";
import crypto from "crypto";
import express from 'express';
import dotenv from 'dotenv';

dotenv.config();

const app = express();

const randomHash = crypto.randomUUID();
const PORT = process.env.PORT ?? '3000';
const PING_PONG_PORT = process.env.PING_PONG_PORT ?? '3001';
const PING_PONG_SERVICE_NAME = process.env.PING_PONG_SERVICE_NAME ?? 'ping-pong-svc';
const PATH = "/tmp/kube/ping.txt";

const randomHashFn = (hash: string) => {
  return `${new Date().toISOString()}: ${hash}`;
}


const printRandomHash = (hash: string) => {
  const count = readFileSync(PATH, { encoding: 'utf8' });
  console.log(`${randomHashFn(hash)}\n Ping/ Pongs: ${count}`)
}
setInterval(printRandomHash, 5000, randomHash);


app.get('/', (_req, res) => {
  const count = readFileSync(PATH, { encoding: 'utf8' });
  return res.send(`${randomHashFn(randomHash)}\nPing / Pongs: ${count}`);
})

app.listen(PORT, () => console.log(`Listening on port: ${PORT}`))


