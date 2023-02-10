import express from "express";
import cors from "cors";
import bodyParser from "body-parser";
import { login, register } from "./auth";
import { initAuth } from "./file/authFile";

const app = express();

app.use(cors);
app.use(bodyParser.json());

app.get("/", (req, res) => {
  res.send("Hello World");
});

initAuth();

app.post("/register", register);
app.post("/login", login);

app.listen(3000, () => {
  console.log("Server is running on port 3000");
});
