import { Request, Response } from "express";
import { newPlayer } from "./file/authFile";

export const register = async (req: Request, res: Response) => {
  const { username, password } = req.body;
  if (!username || !password) {
    return res.status(400).send("Missing username or password");
  }

  if (newPlayer(username, password)) {
    return res.status(200).send("User created");
  }
};

export const login = async (req: Request, res: Response) => {};
