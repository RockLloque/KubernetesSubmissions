import type { Request, Response, NextFunction } from 'express';
import { env } from '../env';
import { readFileSync } from 'fs';

const randomHash = crypto.randomUUID();


export const useLog = (
  _req: Request,
  res: Response,
  next: NextFunction) => {
  const logLine = `${new Date().toISOString()}: ${randomHash}\n`;
  const fileContent = readFileSync("/config/information.txt", { encoding: 'utf8' });
  console.log(`file content: ${fileContent}`);
  console.log(`env variable: MESSAGE=${env.MESSAGE}`);
  console.info(`new Log from middleware: ${logLine}`);
  res.locals.log = logLine;
  res.locals.fileContent = fileContent;
  next()
}
