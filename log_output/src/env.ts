import z from "zod";
import dotenv from 'dotenv';

dotenv.config();




const envSchema = z.object({
  PORT: z.coerce.number().int().positive().default(3000),
  PING_PONG_SERVICE_NAME: z.string().default("ping-pong-svc"),
  PING_PONG_PORT: z.coerce.number().int().positive().default(3001),
  PING_PONG_SUBDIRECTORY: z.string().default("pings"),
  INFORMATION_FILE_PATH: z.string().default('/etc/information/information.txt'),
  LOG_PATH: z.string().default("logs/log.txt")
})


export const env = envSchema.parse(process.env);
