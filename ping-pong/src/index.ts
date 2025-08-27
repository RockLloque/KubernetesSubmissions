import { writeFile } from "fs";
import express from 'express';


const PATH = "/tmp/kube/ping.txt";

const app = express();
const PORT = process.env.PORT ?? 3001;

let count = 0;

app.get("/pingpong", (_req, _res) => {
  ++count;
  writeFile(PATH, `${count}`, { encoding: 'utf8' }, err => {
    if (err) {
      console.log(`Error writing to ${PATH}: ${err}`)
    }
  });
  return `PingPong counter: ${count}`;
});

app.listen(PORT, () => console.log(`PingPong is listening on port: ${PORT}`))
