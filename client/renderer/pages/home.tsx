import React from "react";
import { styled } from "@mui/material";

const Root = styled("div")(({ theme }) => {
  return {
    textAlign: "center",
    paddingTop: theme.spacing(4),
  };
});

function Home() {
  return <h1>Hello World</h1>;
}

export default Home;
