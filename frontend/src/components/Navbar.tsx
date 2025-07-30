import { NavLink } from "react-router-dom";
import { getBaseUrl, setBaseUrl } from "../api";
import { useEffect, useState } from "react";
import BitcoinIcon from "./BitcoinIcon";

export default function Navbar() {
  const [url, setUrl] = useState(getBaseUrl());

  useEffect(() => { setBaseUrl(url); }, [url]);

  const link =
    "px-4 py-2 rounded-lg hover:bg-gray-700 text-sm font-medium transition-all duration-200 flex items-center gap-2";
  const active = "bg-orange-500 text-white hover:bg-orange-600 shadow-lg";

  return (
    <div className="sticky top-0 z-10 bg-gray-800 border-b border-gray-700 shadow-lg">
      <div className="max-w-6xl mx-auto px-4 py-4 flex items-center gap-4">
        <div className="flex items-center gap-3 text-xl font-bold text-white">
          <BitcoinIcon size={32} className="text-orange-500" />
          <span className="bg-gradient-to-r from-orange-400 to-yellow-400 bg-clip-text text-transparent">
            Bitcoin Clone
          </span>
        </div>
        <nav className="flex items-center gap-2">
          <NavLink to="/" end className={({isActive}) => `${link} ${isActive?active:"text-gray-300"}`}>
            <BitcoinIcon size={16} />
            Mining
          </NavLink>
          <NavLink to="/blocks" className={({isActive}) => `${link} ${isActive?active:"text-gray-300"}`}>
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z"/>
            </svg>
            Blocks
          </NavLink>
          <NavLink to="/wallet" className={({isActive}) => `${link} ${isActive?active:"text-gray-300"}`}>
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path d="M4 4a2 2 0 00-2 2v1h16V6a2 2 0 00-2-2H4zM18 9H2v5a2 2 0 002 2h12a2 2 0 002-2V9zM4 13a1 1 0 011-1h1a1 1 0 110 2H5a1 1 0 01-1-1zm5-1a1 1 0 100 2h1a1 1 0 100-2H9z"/>
            </svg>
            Wallet
          </NavLink>
          <NavLink to="/pool" className={({isActive}) => `${link} ${isActive?active:"text-gray-300"}`}>
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clipRule="evenodd"/>
            </svg>
            Pool
          </NavLink>
          <NavLink to="/transact" className={({isActive}) => `${link} ${isActive?active:"text-gray-300"}`}>
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path d="M8 5a1 1 0 100 2h5.586l-1.293 1.293a1 1 0 001.414 1.414l3-3a1 1 0 000-1.414l-3-3a1 1 0 10-1.414 1.414L13.586 5H8zM12 15a1 1 0 100-2H6.414l1.293-1.293a1 1 0 10-1.414-1.414l-3 3a1 1 0 000 1.414l3 3a1 1 0 001.414-1.414L6.414 15H12z"/>
            </svg>
            Transact
          </NavLink>
        </nav>
        <div className="ml-auto flex items-center gap-3">
          <input
            className="w-[320px] bg-gray-700 border border-gray-600 rounded-lg px-4 py-2 text-sm text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-transparent transition-all duration-200"
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            placeholder="http://localhost:3001"
          />
          <span className="text-xs text-gray-400 font-medium">API Base</span>
        </div>
      </div>
    </div>
  );
}