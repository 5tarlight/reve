import { useEffect } from "react";
import ClientBackground from "../components/ClientBackground";
import ReveLogo from "../components/ReveLogo";

const Home = () => {
  useEffect(() => {
    if (localStorage.getItem("login")) {
      window.location.href = "/game";
    } else {
      window.location.href = "/login";
    }
  });

  return (
    <>
      <ClientBackground />
      <ReveLogo />
    </>
  );
};

export default Home;
