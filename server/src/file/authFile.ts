import { randomUUID } from "crypto";
import { existsSync, mkdir, writeFile } from "fs";
import path from "path";

const dir = path.join(__dirname, "..", "data/profiles");

export interface User {
  id: string;
  username: string;
  password: string;
  bp: number;
  rp: number;
  level: number;
  xp: number;
}

export const initAuth = () => {
  mkdir(dir, { recursive: true }, (err) => {
    if (err) throw err;
  });
};

export const newPlayer = (username: string, password: string) => {
  const user: User = {
    id: randomUUID(),
    username,
    password,
    bp: 0,
    rp: 0,
    level: 0,
    xp: 0,
  };

  const json = JSON.stringify(user);
  const file = path.join(dir, `${user.username}.json`);

  if (existsSync(file)) return false;

  writeFile(file, json, (err) => {
    if (err) throw err;
  });

  return true;
};
