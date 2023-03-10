import axios from "axios";
import ClientBackground from "../components/ClientBackground";
import { config } from "../lib/config";

const Game = () => {
  const launch = async () => {
    const res = await axios.get(
      "/api/launch" +
        `?username=${localStorage.getItem("username")}` +
        `&token=${1}` +
        `&room=${1}` +
        `&server=${config.server}` +
        `&team=${"blue"}` +
        `&champion=${"Garen"}`
    );
    console.log(res);
  };

  return (
    <>
      <div>
        <ClientBackground />

        <div className="container">
          <h1>Good to go!</h1>
          <button onClick={launch}>Lauch!</button>
          <div
            className="logout"
            onClick={(e) => {
              e.preventDefault();
              localStorage.removeItem("login");
              localStorage.removeItem("username");
              window.location.href = "/home";
            }}
          >
            Logout
          </div>
        </div>
      </div>
      <style>{`
        * {
          color: white;
        }

        .container {
          max-width: 720px;
          margin: 0 auto;
          display: flex;
          flex-direction: column;
          align-content: center;
          justify-content: center;
        }

        h1 {
          color: white;
          font-size: 50px;
          margin-bottom: 20px;
          margin-top: 20px;
          align-self: center;
        }

        .logout {
          align-self: flex-end;
          margin-top: 20px;
          cursor: pointer;
        }

        .logout:hover {
          color: #bbb;
          text-decoration: underline;
        }

        button {
          align-self: center;
          margin-top: 20px;
          padding: 10px 20px;
          font-size: 20px;
          border: 1px solid white;
          border-radius: 5px;
          background-color: transparent;
          color: white;
          cursor: pointer;
        }

        button:hover {
          background-color: white;
          color: black;
        }
      `}</style>
    </>
  );
};

export default Game;
