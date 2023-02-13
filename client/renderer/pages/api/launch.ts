import { spawn, spawnSync } from "child_process";
import { config } from "../../lib/config";

export default function handler(req, res) {
  const { username, token, room, server, team, champion } = req.query;

  const cmd = `${config.game} ${username} ${token} ${
    server.split("//")[1]
  } ${room} ${team} ${champion}}`;
  const pwd = spawnSync("pwd").output.toString();
  console.log(pwd);
  console.log(cmd);
  const data = spawn(config.game, [
    username,
    token,
    server,
    room,
    team,
    champion,
  ]);

  res.status(200).json({ data: data.stdout.toString() });
}
