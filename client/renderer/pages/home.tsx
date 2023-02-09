import { useEffect } from "react";

const Home = () => {
  useEffect(() => {
    if (localStorage.getItem("login")) {
      window.location.href = "/game";
    } else {
      window.location.href = "/login";
    }
  });
};

export default Home;
