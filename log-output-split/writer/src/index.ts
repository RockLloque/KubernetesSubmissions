import { appendFile } from "fs";

const randomHash = crypto.randomUUID();

const PATH = "/usr/src/app/log/log.txt";



setInterval(() => {
  const logLine = `${new Date().toISOString()}: ${randomHash}\n`;
  appendFile(PATH, logLine, (err) => {
    if (err) {
      console.error(`Error writing ${logLine}: ${err}`);
    } else {
      console.log(logLine);
    }
  });
}, 5000);
