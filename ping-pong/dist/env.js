"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.env = void 0;
const zod_1 = __importDefault(require("zod"));
const dotenv_1 = __importDefault(require("dotenv"));
dotenv_1.default.config();
const envSchema = zod_1.default.object({
    PORT: zod_1.default.coerce.number().default(3001),
    POSTGRES_HOST: zod_1.default.string(),
    POSTGRES_PORT: zod_1.default.coerce.number().default(5432),
    POSTGRES_DB: zod_1.default.string(),
    POSTGRES_USER: zod_1.default.string(),
    POSTGRES_PASSWORD: zod_1.default.string(),
});
exports.env = envSchema.parse(process.env);
//# sourceMappingURL=env.js.map