import { Loader2 } from 'lucide-react';
import { useState } from 'react';

function TransformCard() {
  const API_URL = import.meta.env.VITE_API_URL || '';
  const [inputText, setInputText] = useState('');
  const [result, setResult] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleTransform = async () => {
    if (!inputText.trim()) return;

    setLoading(true);
    setError('');
    setResult('');

    try {
      const res = await fetch(`${API_URL}/api/transform`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text: inputText }),
      });

      if (!res.ok) {
        throw new Error(`API error: ${res.status}`);
      }

      const data = await res.json();
      setResult(data.transformed);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'An error occurred');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="relative z-10 w-full max-w-2xl">
      <div className="bg-white/70 backdrop-blur-md rounded-2xl shadow-lg border border-smoke-200/60 p-6 md:p-8">
        <div className="mb-6">
          <textarea
            className="w-full h-36 bg-smoke-50/80 text-smoke-700 placeholder-smoke-300 rounded-xl p-4 border border-smoke-200 focus:border-amber-500/60 focus:outline-none focus:ring-2 focus:ring-amber-500/20 resize-none transition-all duration-300 text-base"
            placeholder="ここに文章を入力してください..."
            value={inputText}
            onChange={(e) => setInputText(e.target.value)}
          />
        </div>

        <div className="flex justify-center mb-6">
          <button
            type="button"
            onClick={handleTransform}
            disabled={loading || !inputText.trim()}
            className="btn-smoke px-8 py-3 bg-gradient-to-r from-amber-600 to-amber-500 hover:from-amber-500 hover:to-amber-400 disabled:from-smoke-300 disabled:to-smoke-300 disabled:cursor-not-allowed text-white font-bold rounded-xl transition-all duration-300 transform hover:scale-105 disabled:hover:scale-100 text-lg shadow-md"
          >
            {loading ? (
              <span className="flex items-center gap-2">
                <Loader2 className="animate-spin h-5 w-5" />
                変換中...
              </span>
            ) : (
              '変換する \uD83D\uDEAC'
            )}
          </button>
        </div>

        {error && (
          <div className="mb-4 p-4 bg-red-50 border border-red-200 rounded-xl text-red-600 text-sm animate-fade-in">
            {error}
          </div>
        )}

        {result && (
          <div className="animate-fade-in">
            <div className="flex items-center gap-2 mb-3">
              <div className="h-px flex-1 bg-smoke-300/50" />
              <span className="text-smoke-400 text-sm font-medium">
                変換結果
              </span>
              <div className="h-px flex-1 bg-smoke-300/50" />
            </div>
            <div className="bg-smoke-50/60 rounded-xl p-5 border border-smoke-200/40">
              <p className="result-text text-smoke-600 text-lg leading-relaxed">
                {result}
              </p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default TransformCard;
