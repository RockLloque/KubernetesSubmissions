import { appendFile } from "fs";

const randomHash = crypto.randomUUID();

const PATH = "/usr/src/app/log/log.txt";



setInterval(() => {
  const logLine = `${new Date().toISOString()}: ${randomHash}`;
  appendFile(PATH, logLine, (err => console.error(`Error writing ${logLine}: ${err}`)));
}, 5000);
