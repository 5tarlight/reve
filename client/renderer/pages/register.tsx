import ClientBackground from "../components/ClientBackground";
import ReveLogo from "../components/ReveLogo";

const Register = () => {
  return (
    <>
      <ClientBackground />
      <ReveLogo />

      <div className="form">
        <h1>회원가입</h1>
        <input type="text" placeholder="아이디" />
        <input type="password" placeholder="비밀번호" />
        <input type="password" placeholder="비밀번호 확인" />
        <div className="btn-container">
          <div className="other-action">
            <a href="/login">로그인</a>
          </div>
          <button>회원가입</button>
        </div>
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
        `}
      </style>
    </>
  );
};

export default Register;
