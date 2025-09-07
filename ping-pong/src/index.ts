import express from 'express';



const app = express();
const PORT = process.env.PORT ?? 3001;

let count = 0;

app.get("/pingpong", (_req, res) => {
  ++count;

  res.send(`PingPong counter: ${count}`);
});

app.get("/pings", () => {
  return count;
})

app.listen(PORT, () => console.log(`PingPong is listening on port: ${PORT}`))
