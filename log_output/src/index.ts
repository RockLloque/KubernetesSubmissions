import crypto from "crypto";
import express from 'express';
import dotenv from 'dotenv';
import { readFromLog, readFromUrl, writeToLog } from "./volumes/router";
import { env } from './env';
import { useLog } from "./middleware/log";

dotenv.config();

const app = express();


const PORT = env.PORT;
const PING_PONG_PATH = `http://${env.PING_PONG_SERVICE_NAME}:${env.PING_PONG_PORT}/${env.PING_PONG_SUBDIRECTORY}`;


const randomHash = crypto.randomUUID();


const randomHashFn = (hash: string) => {
  return `${new Date().toISOString()}: ${hash}`;
}


app.use("/", useLog);
app.get("/", readFromUrl(PING_PONG_PATH, randomHashFn(randomHash)))

app.get("/logs", readFromLog(env.LOG_PATH))
app.post("/logs", writeToLog(env.LOG_PATH))

app.listen(PORT, () => console.log(`Listening on port: ${PORT}`))


