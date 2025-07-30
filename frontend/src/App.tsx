import { BrowserRouter, Route, Routes } from "react-router-dom";
import Navbar from "./components/Navbar";
import Mining from "./pages/Mining";
import Blocks from "./pages/Blocks";
import Wallet from "./pages/Wallet";
import Pool from "./pages/Pool";
import Transact from "./pages/Transact";

export default function App() {
  // VITE_API_BASE=http://localhost:3001 npm run dev -- --port 5173
  // VITE_API_BASE=http://localhost:3002 npm run dev -- --port 5174
  // VITE_API_BASE=http://localhost:3003 npm run dev -- --port 5175
  return (
    <div className="min-h-screen bg-gray-900">
      <BrowserRouter>
        <Navbar />
        <main className="p-6 bg-gray-900 min-h-screen">
          <Routes>
            <Route path="/" element={<Mining />} />
            <Route path="/blocks" element={<Blocks />} />
            <Route path="/wallet" element={<Wallet />} />
            <Route path="/pool" element={<Pool />} />
            <Route path="/transact" element={<Transact />} />
          </Routes>
        </main>
      </BrowserRouter>
    </div>
  );
}