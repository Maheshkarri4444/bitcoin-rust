import { useEffect, useState } from "react";
import { apiGet } from "../api";
import type { Block, ChainTransaction } from "../types";
import Card from "../components/Card";
import BitcoinIcon from "../components/BitcoinIcon";
import LoadingSpinner from "../components/LoadingSpinner";

function TxView({ tx }: { tx: ChainTransaction }) {
  if (tx.kind === "Reward") {
    const r = tx.tx;
    return (
      <div className="text-xs text-gray-300">
        <div className="font-semibold text-orange-400 flex items-center gap-2 mb-2">
          <BitcoinIcon size={14} />
          Mining Reward
        </div>
        <div className="space-y-1">
          <div><span className="text-gray-500">ID:</span> <span className="font-mono">{r.id}</span></div>
          <div><span className="text-gray-500">Coinbase:</span> <span className="font-mono">{r.coinbase}</span></div>
          <div><span className="text-gray-500">To:</span> <span className="font-mono text-green-400">{r.output.address}</span></div>
          <div><span className="text-gray-500">Amount:</span> <span className="text-orange-400 font-semibold">{r.output.amount} BTC</span></div>
        </div>
      </div>
    );
  }
  const n = tx.tx;
  return (
    <div className="text-xs text-gray-300">
      <div className="font-semibold text-blue-400 flex items-center gap-2 mb-2">
        <svg className="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
          <path d="M8 5a1 1 0 100 2h5.586l-1.293 1.293a1 1 0 001.414 1.414l3-3a1 1 0 000-1.414l-3-3a1 1 0 10-1.414 1.414L13.586 5H8z"/>
        </svg>
        Transaction
      </div>
      <div className="space-y-1">
        <div><span className="text-gray-500">ID:</span> <span className="font-mono">{n.id}</span></div>
        <div><span className="text-gray-500">From:</span> <span className="font-mono text-red-400">{n.input.address}</span></div>
        <div><span className="text-gray-500">Amount In:</span> <span className="text-orange-400 font-semibold">{n.input.amount} BTC</span></div>
        <div><span className="text-gray-500">Timestamp:</span> <span className="text-white">{new Date(n.input.timestamp).toLocaleString()}</span></div>
        <div className="mt-2 text-gray-500">Outputs:</div>
        <ul className="list-none ml-2 space-y-1">
          {n.outputs.map((o, i) => (
            <li key={i} className="break-all flex items-center gap-2">
              <span className="text-green-400">→</span>
              <span className="font-mono text-green-400">{o.address}</span>
              <span className="text-gray-500">:</span>
              <span className="text-orange-400 font-semibold">{o.amount} BTC</span>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}

export default function Blocks() {
  const [blocks, setBlocks] = useState<Block[]>([]);
  const [expanded, setExpanded] = useState<Record<string, boolean>>({});
  const [isLoading, setIsLoading] = useState(true);
  const [refreshLoading, setRefreshLoading] = useState(false);

  const refresh = async () => {
    setRefreshLoading(true);
    try {
      const data = await apiGet<Block[]>("/blocks");
      setBlocks(data);
    } catch (error) {
      console.error("Failed to fetch blocks:", error);
    } finally {
      setRefreshLoading(false);
      setIsLoading(false);
    }
  };
  useEffect(() => { refresh().catch(console.error); }, []);

  if (isLoading) {
    return (
      <div className="max-w-6xl mx-auto space-y-6">
        <Card>
          <div className="text-center py-12">
            <LoadingSpinner size="lg" className="text-orange-500 mx-auto mb-4" />
            <p className="text-gray-400">Loading blockchain data...</p>
          </div>
        </Card>
      </div>
    );
  }

  return (
    <div className="max-w-6xl mx-auto space-y-6">
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
              Refresh Blocks
            </>
          )}
        </button>
        <div className="text-sm text-gray-400 bg-gray-800 px-4 py-2 rounded-lg border border-gray-700">
          <span className="text-orange-400 font-semibold">{blocks.length}</span> blocks in chain
        </div>
      </div>
      
      {blocks.slice().reverse().map((b) => {
        const open = !!expanded[b.hash];
        return (
          <Card key={b.hash}>
            <div className="flex items-center gap-4 mb-4">
              <div className="flex items-center gap-3">
                <div className="text-lg font-bold text-orange-400">#{b.block_number}</div>
                <div className="text-xs text-gray-400 bg-gray-700 px-3 py-1 rounded-full">
                  Difficulty: <span className="text-white font-mono">{b.difficulty}</span>
                </div>
                <div className="text-xs text-gray-400 bg-gray-700 px-3 py-1 rounded-full">
                  Nonce: <span className="text-white font-mono">{b.nonce}</span>
                </div>
              </div>
              <button
                onClick={() => setExpanded((e) => ({ ...e, [b.hash]: !open }))}
                className="ml-auto px-4 py-2 rounded-lg border border-gray-600 text-sm text-gray-300 hover:bg-gray-700 hover:border-orange-500 transition-all duration-200"
              >
                {open ? "Hide Transactions" : `Show Transactions (${b.data?.length ?? 0})`}
              </button>
            </div>
            
            <div className="space-y-2 text-xs">
              <div className="text-gray-400">
                <span className="text-gray-500">Block Hash:</span>
                <div className="font-mono text-green-400 break-all mt-1 bg-gray-900 p-2 rounded">{b.hash}</div>
              </div>
              <div className="text-gray-400">
                <span className="text-gray-500">Previous Hash:</span>
                <div className="font-mono text-blue-400 break-all mt-1 bg-gray-900 p-2 rounded">{b.last_hash}</div>
              </div>
              <div className="text-gray-400">
                <span className="text-gray-500">Timestamp:</span>
                <span className="text-white ml-2">{new Date(b.timestamp).toLocaleString()}</span>
              </div>
            </div>
            
            {open && (
              <div className="mt-6 space-y-4">
                <div className="text-sm font-semibold text-gray-300 border-b border-gray-700 pb-2">
                  Transactions in Block
                </div>
                {b.data?.length
                  ? b.data.map((t, i) => (
                      <div key={i} className="border border-gray-700 rounded-lg p-4 bg-gray-900 hover:bg-gray-800 transition-colors duration-200">
                        <TxView tx={t} />
                      </div>
                    ))
                  : <div className="text-sm text-gray-500 italic text-center py-4">No transactions in this block</div>
                }
              </div>
            )}
          </Card>
        );
      })}
    </div>
  );
}