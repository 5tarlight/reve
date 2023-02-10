import { spawn } from "child_process";
import { config } from "../../lib/config";

export default function handler(req, res) {
  console.log(config.game);
  const data = spawn(config.game);

  res.status(200).json({ name: data.stdout.toString() });
}
