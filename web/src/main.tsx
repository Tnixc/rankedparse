import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { Layout } from "./Layout";
import { CompletionTimes } from "./pages/CompletionTimes";
import { Forfeits } from "./pages/Forfeits";
import { Splits } from "./pages/Splits";
import { Timeline } from "./pages/Timeline";
import "./style.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route element={<Layout />}>
          <Route path="/" element={<Navigate to="/completion-times" replace />} />
          <Route path="/completion-times" element={<CompletionTimes />} />
          <Route path="/forfeits" element={<Forfeits />} />
          <Route path="/splits" element={<Splits />} />
          <Route path="/timeline" element={<Timeline />} />
        </Route>
      </Routes>
    </BrowserRouter>
  </React.StrictMode>,
);
