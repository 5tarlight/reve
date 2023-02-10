import { Request, Response } from "express";
import { getUser, newPlayer } from "./file/authFile";

export const register = async (req: Request, res: Response) => {
  const { username, password } = req.body;
  if (!username || !password) {
    return res.status(400).send("Missing username or password");
  }

  if (newPlayer(username, password)) {
    return res.status(200).send("User created");
  } else {
    return res.status(400).send("User already exists");
  }
};

export const login = async (req: Request, res: Response) => {
  const { username, password } = req.body;
  if (!username || !password) {
    return res.status(400).send("Missing username or password");
  }

  const user = await getUser(username);
  if (!user) {
    return res.status(400).send("User does not exist");
  }

  if (user.password === password) {
    return res.status(200).send("User logged in");
  } else {
    return res.status(400).send("Wrong password");
  }
};
