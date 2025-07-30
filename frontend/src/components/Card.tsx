import { PropsWithChildren } from "react";

export default function Card({ children }: PropsWithChildren) {
  return (
    <div className="p-6 bg-gray-800 rounded-2xl shadow-xl border border-gray-700 hover:border-orange-500/30 transition-all duration-300">
      {children}
    </div>
  );
}