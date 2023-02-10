import { spawn, spawnSync } from "child_process";
import { config } from "../../lib/config";

export default function handler(req, res) {
  const { username, token, room, server } = req.query;

  const cmd = `${config.game} ${username} ${token} ${
    server.split("//")[1]
  } ${room}`;
  const pwd = spawnSync("pwd").output.toString();
  console.log(pwd);
  console.log(cmd);
  const data = spawn(config.game, [username, token, server, room]);

  res.status(200).json({ name: data.stdout.toString() });
}
