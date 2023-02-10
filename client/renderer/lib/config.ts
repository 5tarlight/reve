import dotenv from "dotenv";

dotenv.config();

export const config = {
  server: `http://${process.env.server || "localhost"}`,
};
