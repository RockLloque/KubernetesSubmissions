import express from 'express';

const app = express();

const randomHash = crypto.randomUUID();
const port = '3000';

const randomHashFn = (hash) => {
  return `${new Date().toISOString()}: ${hash}`;
}


const printRandomHash = (hash) => {
  console.log(randomHashFn(hash))
}
setInterval(printRandomHash, 5000, randomHash);


app.get('/', (_req, res) => res.send(randomHash(randomHash)))

app.listen(port, () => console.log(`Listening on port: ${port}`))


