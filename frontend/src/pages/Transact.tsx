import { useState } from "react";
import { apiPost } from "../api";
import Card from "../components/Card";
import BitcoinIcon from "../components/BitcoinIcon";
import LoadingSpinner from "../components/LoadingSpinner";

export default function Transact() {
  const [recipient, setRecipient] = useState("");
  const [amount, setAmount] = useState<number>(0);
  const [msg, setMsg] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const submit = async () => {
    setMsg("");
    setIsLoading(true);
    try {
      await apiPost("/transact", { recipient, amount: Number(amount) });
      setMsg("Transaction submitted successfully! Check Pool/Blocks for confirmation.");
      setRecipient("");
      setAmount(0);
    } catch (e: any) {
      setMsg(e.message || "Transaction failed");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="max-w-3xl mx-auto space-y-6">
      <Card>
        <div className="text-center mb-6">
          <div className="flex items-center justify-center gap-3 mb-4">
            <BitcoinIcon size={32} className="text-orange-500" />
            <h2 className="text-2xl font-bold text-white">Send Bitcoin</h2>
          </div>
          <p className="text-gray-400">Create and broadcast a new transaction to the network</p>
        </div>
        
        <div className="space-y-6">
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Recipient Public Key
            </label>
            <input
              className="w-full bg-gray-700 border border-gray-600 rounded-lg px-4 py-3 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-transparent transition-all duration-200 font-mono text-sm"
              placeholder="Enter recipient's public key address..."
              value={recipient}
              onChange={(e) => setRecipient(e.target.value)}
            />
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Amount (BTC)
            </label>
            <div className="relative">
              <input
                className="w-full bg-gray-700 border border-gray-600 rounded-lg px-4 py-3 pr-16 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-transparent transition-all duration-200"
                type="number"
                step="0.00000001"
                min="0"
                placeholder="0.00000000"
                value={amount || ""}
                onChange={(e) => setAmount(Number(e.target.value))}
              />
              <div className="absolute right-3 top-1/2 transform -translate-y-1/2 text-orange-400 font-semibold">
                BTC
              </div>
            </div>
          </div>
          
          <div className="flex items-center gap-4">
            <button 
              onClick={submit} 
              disabled={isLoading || !recipient || !amount}
              className="flex-1 px-6 py-3 rounded-lg bg-orange-500 hover:bg-orange-600 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-semibold transition-all duration-200 shadow-lg hover:shadow-xl flex items-center justify-center gap-2"
            >
              {isLoading ? (
                <>
                  <LoadingSpinner size="sm" />
                  Processing...
                </>
              ) : (
                <>
                  <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M8 5a1 1 0 100 2h5.586l-1.293 1.293a1 1 0 001.414 1.414l3-3a1 1 0 000-1.414l-3-3a1 1 0 10-1.414 1.414L13.586 5H8zM12 15a1 1 0 100-2H6.414l1.293-1.293a1 1 0 10-1.414-1.414l-3 3a1 1 0 000 1.414l3 3a1 1 0 001.414-1.414L6.414 15H12z"/>
                  </svg>
                  Send Transaction
                </>
              )}
            </button>
          </div>
          
          {msg && (
            <div className={`p-4 rounded-lg border ${
              msg.includes("successfully") 
                ? "bg-green-900 border-green-700 text-green-300" 
                : "bg-red-900 border-red-700 text-red-300"
            }`}>
              <div className="flex items-center gap-2">
                {msg.includes("successfully") ? (
                  <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                    <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd"/>
                  </svg>
                ) : (
                  <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                    <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clipRule="evenodd"/>
                  </svg>
                )}
                <span className="font-medium">{msg}</span>
              </div>
            </div>
          )}
        </div>
      </Card>
      
      <Card>
        <h3 className="text-lg font-semibold text-white mb-4 flex items-center gap-2">
          <svg className="w-5 h-5 text-orange-500" fill="currentColor" viewBox="0 0 20 20">
            <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clipRule="evenodd"/>
          </svg>
          Transaction Info
        </h3>
        <div className="text-sm text-gray-400 space-y-2">
          <p>• Transactions are broadcast to the network and added to the transaction pool</p>
          <p>• Miners will include your transaction in the next block they mine</p>
          <p>• Check the Pool tab to see pending transactions</p>
          <p>• Check the Blocks tab to see confirmed transactions</p>
        </div>
      </Card>
    </div>
  );
}