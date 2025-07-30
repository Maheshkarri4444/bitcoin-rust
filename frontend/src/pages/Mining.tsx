import { useEffect, useState } from "react";
import { apiGet } from "../api";
import Card from "../components/Card";
import BitcoinIcon from "../components/BitcoinIcon";
import LoadingSpinner from "../components/LoadingSpinner";

export default function Mining() {
  const [running, setRunning] = useState(false);
  // const [status, setStatus] = useState("");
  const [isLoading, setIsLoading] = useState(true);
  const [startLoading, setStartLoading] = useState(false);
  const [stopLoading, setStopLoading] = useState(false);

  const fetchStatus = async () => {
    try {
      const s = await apiGet<{ running: boolean }>("/mining-status");
      setRunning(s.running);
    } catch {
      // ignore
    }
  };

  const initialLoad = async () => {
    try {
      await fetchStatus();
    } finally {
      setIsLoading(false);
    }
  };

  const start = async () => {
    setStartLoading(true);
    // setStatus("Starting mining operation...");
    try {
      await apiGet("/startmining");
      await fetchStatus();
      // setStatus("Mining started successfully");
      // Clear success message after 3 seconds
      // setTimeout(() => setStatus(""), 3000);
    } catch (error) {
      // setStatus("Failed to start mining");
      // Clear error message after 5 seconds
      // setTimeout(() => setStatus(""), 5000);
    } finally {
      setStartLoading(false);
    }
  };

  const stop = async () => {
    setStopLoading(true);
    // setStatus("Stopping mining operation...");
    try {
      await apiGet("/stopmining");
      await fetchStatus();
      // setStatus("Mining stopped");
      // Clear success message after 3 seconds
      // setTimeout(() => setStatus(""), 3000);
    } catch (error) {
      // setStatus("Failed to stop mining");
      // Clear error message after 5 seconds
      // setTimeout(() => setStatus(""), 5000);
    } finally {
      setStopLoading(false);
    }
  };

  useEffect(() => {
    initialLoad();
    const id = setInterval(fetchStatus, 2000);
    return () => clearInterval(id);
  }, []);

  if (isLoading) {
    return (
      <div className="max-w-4xl mx-auto space-y-6">
        <Card>
          <div className="text-center py-12">
            <LoadingSpinner size="lg" className="text-orange-500 mx-auto mb-4" />
            <p className="text-gray-400">Loading mining status...</p>
          </div>
        </Card>
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <Card>
        <div className="text-center mb-6">
          <div className="flex items-center justify-center gap-3 mb-4">
            <BitcoinIcon size={48} className="text-orange-500" />
            <h1 className="text-3xl font-bold text-white">Bitcoin Mining</h1>
          </div>
          <p className="text-gray-400">Control your mining operations and monitor status</p>
        </div>
        
        <div className="flex items-center justify-center gap-6 mb-6">
          <button
            onClick={start}
            disabled={running || startLoading}
            className={`px-8 py-4 rounded-lg text-white font-semibold transition-all duration-200 shadow-lg hover:shadow-xl flex items-center gap-3 ${
              running || startLoading
                ? "bg-gray-600 cursor-not-allowed" 
                : "bg-green-600 hover:bg-green-700 transform hover:scale-105"
            }`}
          >
            {startLoading ? (
              <>
                <LoadingSpinner size="sm" />
                Starting...
              </>
            ) : (
              <>
                <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                  <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd"/>
                </svg>
                Start Mining
              </>
            )}
          </button>
          
          <button
            onClick={stop}
            disabled={!running || stopLoading}
            className={`px-8 py-4 rounded-lg text-white font-semibold transition-all duration-200 shadow-lg hover:shadow-xl flex items-center gap-3 ${
              !running || stopLoading
                ? "bg-gray-600 cursor-not-allowed" 
                : "bg-red-600 hover:bg-red-700 transform hover:scale-105"
            }`}
          >
            {stopLoading ? (
              <>
                <LoadingSpinner size="sm" />
                Stopping...
              </>
            ) : (
              <>
                <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                  <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8 7a1 1 0 00-1 1v4a1 1 0 001 1h4a1 1 0 001-1V8a1 1 0 00-1-1H8z" clipRule="evenodd"/>
                </svg>
                Stop Mining
              </>
            )}
          </button>
        </div>
        
        <div className="flex items-center justify-center gap-6">
          {status && (
            <div className="text-sm text-gray-300 bg-gray-700 px-4 py-2 rounded-lg border border-gray-600">
              {status}
            </div>
          )}
          
          <div className={`flex items-center gap-2 px-4 py-2 rounded-lg font-medium ${
            running 
              ? "bg-green-900 text-green-300 border border-green-700" 
              : "bg-gray-700 text-gray-300 border border-gray-600"
          }`}>
            <div className={`w-3 h-3 rounded-full ${running ? "bg-green-400 animate-pulse" : "bg-gray-500"}`}></div>
            {running ? "Mining Active" : "Mining Idle"}
          </div>
        </div>
      </Card>
      
      <Card>
        <h2 className="text-xl font-semibold text-white mb-4 flex items-center gap-2">
          <svg className="w-5 h-5 text-orange-500" fill="currentColor" viewBox="0 0 20 20">
            <path fillRule="evenodd" d="M3 3a1 1 0 000 2v8a2 2 0 002 2h2.586l-1.293 1.293a1 1 0 101.414 1.414L10 15.414l2.293 2.293a1 1 0 001.414-1.414L12.414 15H15a2 2 0 002-2V5a1 1 0 100-2H3zm11.707 4.707a1 1 0 00-1.414-1.414L10 9.586 8.707 8.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd"/>
          </svg>
          Mining Information
        </h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
          <div className="bg-gray-900 p-4 rounded-lg border border-gray-700">
            <div className="text-gray-400 mb-1">Status</div>
            <div className="text-white font-semibold">{running ? "Active" : "Inactive"}</div>
          </div>
          <div className="bg-gray-900 p-4 rounded-lg border border-gray-700">
            <div className="text-gray-400 mb-1">Algorithm</div>
            <div className="text-white font-semibold">SHA-256</div>
          </div>
          <div className="bg-gray-900 p-4 rounded-lg border border-gray-700">
            <div className="text-gray-400 mb-1">Network</div>
            <div className="text-white font-semibold">Bitcoin Clone</div>
          </div>
        </div>
      </Card>
    </div>
  );
}