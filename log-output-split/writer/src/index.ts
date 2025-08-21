
import express from 'express';



const app = express();
const PORT = process.env.PORT ?? 3001;

let count = 0;

app.get("/pingpong", (_req, res) => `PingPong counter: ${++count}`);
app.listen(PORT, () => console.log(`PingPong is listening on port: ${PORT}`))

