import { useEffect, useState } from "react";
import { apiGet } from "../api";
import type { BalanceResp, PublicKeyResp } from "../types";
import Card from "../components/Card";
import BitcoinIcon from "../components/BitcoinIcon";
import LoadingSpinner from "../components/LoadingSpinner";

export default function Wallet() {
  const [balance, setBalance] = useState<number>(0);
  const [publicKey, setPublicKey] = useState<string>("");
  const [isLoading, setIsLoading] = useState(true);
  const [refreshLoading, setRefreshLoading] = useState(false);

  const refresh = async () => {
    setRefreshLoading(true);
    try {
      const [balanceResp, keyResp] = await Promise.all([
        apiGet<BalanceResp>("/balance"),
        apiGet<PublicKeyResp>("/publickey")
      ]);
      setBalance(balanceResp.balance);
      setPublicKey(keyResp.public_key);
    } catch (error) {
      console.error("Failed to fetch wallet data:", error);
    } finally {
      setRefreshLoading(false);
      setIsLoading(false);
    }
  };

  useEffect(() => { 
    refresh().catch(console.error); 
  }, []);

  if (isLoading) {
    return (
      <div className="max-w-4xl mx-auto space-y-6">
        <Card>
          <div className="text-center py-12">
            <LoadingSpinner size="lg" className="text-orange-500 mx-auto mb-4" />
            <p className="text-gray-400">Loading wallet information...</p>
          </div>
        </Card>
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <div className="flex items-center gap-4">
        <button 
          onClick={refresh} 
          disabled={refreshLoading}
          className="px-6 py-3 rounded-lg bg-orange-500 hover:bg-orange-600 text-white font-medium transition-all duration-200 shadow-lg hover:shadow-xl flex items-center gap-2"
        >
          {refreshLoading ? (
            <>
              <LoadingSpinner size="sm" />
              Refreshing...
            </>
          ) : (
            <>
              <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                <path fillRule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clipRule="evenodd"/>
              </svg>
              Refresh Wallet
            </>
          )}
        </button>
      </div>

      <Card>
        <div className="text-center mb-6">
          <div className="flex items-center justify-center gap-3 mb-4">
            <BitcoinIcon size={48} className="text-orange-500" />
            <h1 className="text-3xl font-bold text-white">Bitcoin Wallet</h1>
          </div>
          <p className="text-gray-400">Manage your Bitcoin balance and public key</p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div className="bg-gray-900 p-6 rounded-lg border border-gray-700">
            <div className="flex items-center gap-3 mb-4">
              <BitcoinIcon size={24} className="text-orange-500" />
              <h3 className="text-lg font-semibold text-white">Balance</h3>
            </div>
            <div className="text-3xl font-bold text-orange-400 mb-2">
              {balance.toFixed(8)} BTC
            </div>
            <div className="text-sm text-gray-400">
              Current wallet balance
            </div>
          </div>

          <div className="bg-gray-900 p-6 rounded-lg border border-gray-700">
            <div className="flex items-center gap-3 mb-4">
              <svg className="w-6 h-6 text-orange-500" fill="currentColor" viewBox="0 0 20 20">
                <path fillRule="evenodd" d="M18 8a6 6 0 01-7.743 5.743L10 14l-1 1-1 1H6v2H2v-4l4.257-4.257A6 6 0 1118 8zm-6-4a1 1 0 100 2 2 2 0 012 2 1 1 0 102 0 4 4 0 00-4-4z" clipRule="evenodd"/>
              </svg>
              <h3 className="text-lg font-semibold text-white">Public Key</h3>
            </div>
            <div className="font-mono text-sm text-green-400 break-all bg-gray-800 p-3 rounded border border-gray-600">
              {publicKey || "Loading..."}
            </div>
            <div className="text-sm text-gray-400 mt-2">
              Your public address for receiving Bitcoin
            </div>
          </div>
        </div>
      </Card>

      <Card>
        <h2 className="text-xl font-semibold text-white mb-4 flex items-center gap-2">
          <svg className="w-5 h-5 text-orange-500" fill="currentColor" viewBox="0 0 20 20">
            <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clipRule="evenodd"/>
          </svg>
          Wallet Information
        </h2>
        <div className="text-sm text-gray-400 space-y-2">
          <p>• Your public key is used to receive Bitcoin from other users</p>
          <p>• Share your public key with others to receive payments</p>
          <p>• Your balance updates automatically when transactions are confirmed</p>
          <p>• Use the Transact tab to send Bitcoin to other addresses</p>
        </div>
      </Card>
    </div>
  );
}