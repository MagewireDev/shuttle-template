'use client'

import { useState } from "react";
import Button from "@mui/material/Button";

export default function Home() {
  const [status, setStatus] = useState<string>("");

  const checkHealth = async () => {
    const res = await fetch(`//${window.location.host}/health`);
    const data = await res.json();
    setStatus(data.status);
  };

  return (
    <div style={{ padding: 40 }}>
      <Button variant="contained" onClick={checkHealth}>
        Ping Health
      </Button>
      {status && <p>Status: {status}</p>}
    </div>
  );
}
