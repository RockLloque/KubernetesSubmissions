import express from 'express';
import { readFileSync } from 'fs';

const app = express();
const PORT = process.env.PORT ?? 3000;
const PATH = "/usr/src/app/log/log.txt";

app.get("/logs", (_req, res) => {
  const log = readFileSync(PATH, { encoding: 'utf8' });
  console.log(log);
  return res.send(`<pre>${log}</pre>`);
});

app.listen(PORT, () => console.log(`Reader app is listening on port: ${PORT}`));

