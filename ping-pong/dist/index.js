"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const express_1 = __importDefault(require("express"));
const db_1 = require("./db");
const env_1 = require("./env");
const app = (0, express_1.default)();
const PORT = env_1.env.PORT;
app.get("/pingpong", async (_req, res) => {
    try {
        const count = await (0, db_1.incrementCount)();
        res.send(`PingPong counter: ${count}`);
    }
    catch (error) {
        console.error('Error incrementing counter:', error);
        res.status(500).send('Failed to increment counter');
    }
});
app.get("/pings", async (_req, res) => {
    try {
        const count = await (0, db_1.getCount)();
        res.send(count.toString());
    }
    catch (error) {
        console.error('Error getting counter:', error);
        res.status(500).send('Failed to get counter');
    }
});
app.get("/health", async (_req, res) => {
    const dbHealthy = await (0, db_1.checkDatabaseHealth)();
    if (dbHealthy) {
        res.status(200).json({ status: 'healthy', database: 'connected' });
    }
    else {
        res.status(503).json({ status: 'unhealthy', database: 'disconnected' });
    }
});
async function startServer() {
    try {
        console.log('Initializing database...');
        await (0, db_1.initializeDatabase)();
        console.log('Database ready');
        app.listen(PORT, () => {
            console.log(`PingPong is listening on port: ${PORT}`);
        });
    }
    catch (error) {
        console.error('Failed to start server:', error);
        process.exit(1);
    }
}
process.on('SIGTERM', async () => {
    console.log('SIGTERM received, closing database connection...');
    await (0, db_1.closeDatabaseConnection)();
    process.exit(0);
});
process.on('SIGINT', async () => {
    console.log('SIGINT received, closing database connection...');
    await (0, db_1.closeDatabaseConnection)();
    process.exit(0);
});
startServer();
//# sourceMappingURL=index.js.map