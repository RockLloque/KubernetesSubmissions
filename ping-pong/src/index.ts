import express from 'express';
import dotenv from 'dotenv';

dotenv.config();

const app = express();
const PORT = process.env.PORT ?? 3001;

let count = 0;

app.get("/pingpong", (_req, res) => {
  ++count;

  res.send(`PingPong counter: ${count}`);
});

app.get("/pings", (_req, res) => {
  res.send(count.toString());
})

app.listen(PORT, () => console.log(`PingPong is listening on port: ${PORT}`))
