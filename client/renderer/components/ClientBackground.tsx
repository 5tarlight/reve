const ClientBackground = () => {
  return (
    <>
      <div className="background" />
      <style jsx>
        {`
          .background {
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background-color: #202020;
            z-index: -1;
          }
        `}
      </style>
    </>
  );
};

export default ClientBackground;
