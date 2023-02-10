import axios from "axios";
import { useState } from "react";
import ClientBackground from "../components/ClientBackground";
import ReveLogo from "../components/ReveLogo";
import { config } from "../lib/config";

const Register = () => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [passwordConfirm, setPasswordConfirm] = useState("");
  const [message, setMessage] = useState("");

  const handleClick = async () => {
    if (!username || !password || !passwordConfirm) {
      setMessage("모든 항목을 입력해주세요.");
      return;
    }

    if (password !== passwordConfirm) {
      setMessage("비밀번호가 일치하지 않습니다.");
      return;
    }

    try {
      const res = await axios.post(config.server + "/register", {
        username,
        password,
      });

      if (res.status === 200) {
        setMessage(
          "회원가입에 성공했습니다. 로그인 페이지에서 로그인 할 수 있습니다."
        );
      } else {
        setMessage("회원가입에 실패했습니다.");
      }
    } catch (e) {
      setMessage("회원가입에 실패했습니다.");
    }
  };

  return (
    <>
      <ClientBackground />
      <ReveLogo />

      <div className="form">
        <h1>회원가입</h1>
        <input
          type="text"
          placeholder="아이디"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
        />

        <input
          type="password"
          placeholder="비밀번호"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
        <input
          type="password"
          placeholder="비밀번호 확인"
          value={passwordConfirm}
          onChange={(e) => setPasswordConfirm(e.target.value)}
        />
        <div className="btn-container">
          <div className="other-action">
            <a href="/login">로그인</a>
          </div>
          <button
            onClick={(e) => {
              e.preventDefault();
              e.stopPropagation();
              handleClick();
            }}
          >
            회원가입
          </button>
        </div>
        <div className="message">{message}</div>
      </div>

      <style jsx>
        {`
          .form {
            max-width: 720px;
            margin: 0 auto;
            display: flex;
            flex-direction: column;
          }

          h1 {
            color: white;
            font-size: 50px;
            margin-bottom: 20px;
          }

          input,
          input:focus {
            width: 100%;
            height: 50px;
            border: none;
            border-radius: 5px;
            margin-bottom: 20px;
            padding: 0 20px;
            font-size: 24px;
            outline: none;
            background-color: transparent;
            color: white;
            border-bottom: 2px solid white;
          }

          .btn-container {
            display: flex;
            justify-content: space-between;
          }

          button {
            width: 100px;
            height: 50px;
            border: none;
            border-radius: 5px;
            background-color: white;
            color: black;
            font-size: 24px;
            cursor: pointer;
          }

          button:hover {
            background-color: #f54248;
            color: white;
          }

          .other-action {
            display: flex;
            align-items: center;
          }

          .other-action * {
            color: white;
            font-size: 20px;
          }

          .other-action a:hover {
            text-decoration: underline;
          }

          .message {
            color: white;
          }
        `}
      </style>
    </>
  );
};

export default Register;
