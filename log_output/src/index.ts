import crypto from "crypto";
import express from 'express';
import dotenv from 'dotenv';
import { readFromLog, writeToLog } from "./volumes/router";
import { env } from './env';

dotenv.config();

const app = express();


const PORT = env.PORT;
const PING_PONG_PATH = `http://${env.PING_PONG_SERVICE_NAME}:${env.PING_PONG_PORT}/${env.PING_PONG_SUBDIRECTORY}`;


const randomHash = crypto.randomUUID();


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

app.get("/logs", readFromLog(env.LOG_PATH))
app.post("/logs", writeToLog(env.LOG_PATH))

app.listen(PORT, () => console.log(`Listening on port: ${PORT}`))


