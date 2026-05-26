import Create from "./pages/Create";

function App() {
  const isAuthenticated = true; // TODO: wire to wallet/session

  if (!isAuthenticated) {
    return (
      <div style={{ minHeight: "100vh", background: "#0f0f23", color: "#fff", display: "flex", alignItems: "center", justifyContent: "center", fontFamily: "sans-serif" }}>
        <div style={{ textAlign: "center" }}>
          <h2>Connect your wallet to create a puzzle</h2>
          <p style={{ color: "#888" }}>You must be authenticated to access /create.</p>
        </div>
      </div>
    );
  }

  return <Create />;
}

export default App;
