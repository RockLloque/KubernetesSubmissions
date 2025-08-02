const randomHash = crypto.randomUUID();


const printRandomHash = (hash) => {
  console.log(`${new Date().toISOString()}: ${hash}`);
}

setInterval(printRandomHash, 5000, randomHash);
