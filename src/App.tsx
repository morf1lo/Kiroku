import "./App.css";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Button from "./components/Button";

type HistoryItem =
  | { type: "Text"; data: string }
  | { type: "Image"; data: string };

function App() {
  const [history, setHistory] = useState<HistoryItem[]>([]);
  const [previewImage, setPreviewImage] = useState<string | null>(null);

  useEffect(() => {
    const interval = setInterval(() => {
      invoke<HistoryItem[]>("get_history").then(setHistory);
    }, 2000);
    return () => clearInterval(interval);
  }, []);

  const copy = (item: HistoryItem) => {
    invoke("copy_to_clipboard", { item });
  };

  const clearHistory = () => {
    invoke<HistoryItem[]>("clear_history").then(() => setHistory([]));
  };

  const imagePreview = (item: HistoryItem) => {
    setPreviewImage(item.data);
  };

  return (
    <div className="app">
      <div className="header">
        <Button text="Clear history" onClick={clearHistory} />
      </div>
      <div className="items">
        {history.map((item, i) => (
          <div className="item" key={i}>
            {item.type === "Text" ? (
              <>
                <div className="item-copy">
                  <p style={{ color: "#f1f1f1", fontFamily: "monospace" }}>TEXT</p>
                  <Button text="Copy" onClick={() => copy(item)} />
                </div>
                <p className="item-text" style={{ fontFamily: "cursive" }}>
                  {item.data.length > 200 ? (
                    <>{item.data.slice(0, 200) + " ..."}</>
                  ) : (
                    <>{item.data}</>
                  )}
                </p>
              </>
            ) : (
              <>
                <div className="item-copy">
                  <p style={{ color: "#f1f1f1", fontFamily: "monospace" }}>IMAGE</p>
                  <div style={{ display: "flex", "gap": "5px" }}>
                    <Button text="Copy" onClick={() => copy(item)} />
                    <Button text="Preview" onClick={() => imagePreview(item)} />
                  </div>
                </div>
              </>
            )}
          </div>
        ))}
      </div>

      {previewImage && (
        <div
          className="overlay"
          onClick={() => setPreviewImage(null)}
          style={{
            position: "fixed",
            top: 0,
            left: 0,
            width: "100vw",
            height: "100vh",
            backgroundColor: "rgba(0,0,0,0.85)",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            zIndex: 1000,
          }}
        >
          <img
            src={`data:image/png;base64,${previewImage}`}
            alt="Preview"
            style={{
              maxWidth: "90%",
              maxHeight: "90%",
              borderRadius: "8px",
              boxShadow: "0 0 20px rgba(0,0,0,0.8)",
              userSelect: "none",
            }}
            onClick={(e) => e.stopPropagation()}
          />
        </div>
      )}
    </div>
  );
}

export default App;
