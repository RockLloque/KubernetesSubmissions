import type { Request, Response } from 'express';
import { readFileSync, writeFileSync, appendFileSync } from 'fs';


const randomHash = crypto.randomUUID();

export const writeToLog = (path: string) => (_req: Request, res: Response) => {
  const logLine = `${new Date().toISOString()}: ${randomHash}\n`;
  let message = '';
  try {
    appendFileSync(path, logLine, { encoding: 'utf8' });
    message = `Wrote ${logLine} to ${path}`;
  } catch (err: any) {
    message = `Error: Could not write data to path: ${path}`;
    console.error(message);
  }
  res.send(`<pre>${message}</pre>`);
}


export const readFromLog = (path: string) => (_req: Request, res: Response) => {
  const log = readFileSync(path, { encoding: 'utf8' });
  console.log(`Read from file ${path}: ${log}`);
  return res.send(`<pre>${log}</pre>`);
}
