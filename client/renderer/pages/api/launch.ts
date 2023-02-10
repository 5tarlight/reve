import { spawn } from "child_process";
import { config } from "../../lib/config";

export default function handler(req, res) {
  const { username, token, room, server } = req.query;

  const cmd = `${config.game} ${username} ${token} ${server} ${room}`;
  console.log(cmd);
  const data = spawn(cmd);

  res.status(200).json({ name: data.stdout.toString() });
}
