import { useEffect, useState } from "react";
import { apiGet } from "../api";
import type { ChainTransaction } from "../types";
import Card from "../components/Card";
import LoadingSpinner from "../components/LoadingSpinner";

export default function Pool() {
  const [pool, setPool] = useState<ChainTransaction[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [refreshLoading, setRefreshLoading] = useState(false);

  const refresh = async () => {
    setRefreshLoading(true);
    try {
      const p = await apiGet<ChainTransaction[]>("/transactions");
      setPool(p);
    } catch (error) {
      console.error("Failed to fetch pool:", error);
    } finally {
      setRefreshLoading(false);
      setIsLoading(false);
    }
  };

  useEffect(() => { refresh().catch(console.error); }, []);

  if (isLoading) {
    return (
      <div className="max-w-5xl mx-auto space-y-6">
        <Card>
          <div className="text-center py-12">
            <LoadingSpinner size="lg" className="text-orange-500 mx-auto mb-4" />
            <p className="text-gray-400">Loading transaction pool...</p>
          </div>
        </Card>
      </div>
    );
  }

  return (
    <div className="max-w-5xl mx-auto space-y-6">
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
              Refresh Pool
            </>
          )}
        </button>
        <div className="text-sm text-gray-400 bg-gray-800 px-4 py-2 rounded-lg border border-gray-700">
          <span className="text-orange-400 font-semibold">{pool.length}</span> pending transactions
        </div>
      </div>
      
      <Card>
        <div className="flex items-center gap-3 mb-4">
          <svg className="w-6 h-6 text-orange-500" fill="currentColor" viewBox="0 0 20 20">
            <path fillRule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clipRule="evenodd"/>
          </svg>
          <h2 className="text-xl font-semibold text-white">Transaction Pool</h2>
        </div>
        <p className="text-gray-400 mb-6">Pending transactions waiting to be mined into blocks</p>
        
        {pool.length === 0 ? (
          <div className="text-center py-12">
            <svg className="w-16 h-16 text-gray-600 mx-auto mb-4" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clipRule="evenodd"/>
            </svg>
            <h3 className="text-lg font-medium text-gray-400 mb-2">No Pending Transactions</h3>
            <p className="text-gray-500">The transaction pool is empty. All transactions have been mined.</p>
          </div>
        ) : (
          <div className="bg-gray-900 rounded-lg border border-gray-700 overflow-hidden">
            <pre className="text-xs text-gray-300 overflow-auto max-h-[70vh] p-4 font-mono leading-relaxed">
              {JSON.stringify(pool, null, 2)}
            </pre>
          </div>
        )}
      </Card>
    </div>
  );
}