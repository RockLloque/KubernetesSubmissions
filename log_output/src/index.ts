import { readFileSync } from "fs";
import crypto from "crypto";
import express from 'express';

const app = express();

const randomHash = crypto.randomUUID();
const port = '3000';
const PATH = "/tmp/kube/ping.txt";

const randomHashFn = (hash: string) => {
  return `${new Date().toISOString()}: ${hash}`;
}


const printRandomHash = (hash: string) => {
  console.log(randomHashFn(hash))
}
setInterval(printRandomHash, 5000, randomHash);


app.get('/', (_req, res) => {
  const count = readFileSync(PATH, { encoding: 'utf8' });
  return res.send(`${randomHashFn(randomHash)}\nPing / Pongs: ${count}`);
})

app.listen(port, () => console.log(`Listening on port: ${port}`))


