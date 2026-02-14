import { useState } from 'react'

function TransformCard() {
  const API_URL = import.meta.env.VITE_API_URL || '';
  const [inputText, setInputText] = useState('')
  const [result, setResult] = useState('')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')

  const handleTransform = async () => {
    if (!inputText.trim()) return

    setLoading(true)
    setError('')
    setResult('')

    try {
      const res = await fetch(`${API_URL}/api/transform`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text: inputText }),
      })

      if (!res.ok) {
        throw new Error(`API error: ${res.status}`)
      }

      const data = await res.json()
      setResult(data.transformed)
    } catch (e) {
      setError(e instanceof Error ? e.message : 'An error occurred')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="relative z-10 w-full max-w-2xl">
      <div className="bg-smoke-800/80 backdrop-blur-md rounded-2xl shadow-2xl border border-smoke-600/30 p-6 md:p-8">
        <div className="mb-6">
          <textarea
            className="w-full h-36 bg-smoke-700/50 text-smoke-100 placeholder-smoke-400 rounded-xl p-4 border border-smoke-600/40 focus:border-amber-500/60 focus:outline-none focus:ring-2 focus:ring-amber-500/20 resize-none transition-all duration-300 text-base"
            placeholder="ここに文章を入力してください..."
            value={inputText}
            onChange={(e) => setInputText(e.target.value)}
          />
        </div>

        <div className="flex justify-center mb-6">
          <button
            onClick={handleTransform}
            disabled={loading || !inputText.trim()}
            className="btn-smoke px-8 py-3 bg-gradient-to-r from-amber-600 to-amber-500 hover:from-amber-500 hover:to-amber-400 disabled:from-smoke-600 disabled:to-smoke-600 disabled:cursor-not-allowed text-white font-bold rounded-xl transition-all duration-300 transform hover:scale-105 disabled:hover:scale-100 text-lg shadow-lg"
          >
            {loading ? (
              <span className="flex items-center gap-2">
                <svg className="animate-spin h-5 w-5" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
                </svg>
                変換中...
              </span>
            ) : (
              '変換する \uD83D\uDEAC'
            )}
          </button>
        </div>

        {error && (
          <div className="mb-4 p-4 bg-red-900/30 border border-red-500/40 rounded-xl text-red-300 text-sm animate-fade-in">
            {error}
          </div>
        )}

        {result && (
          <div className="animate-fade-in">
            <div className="flex items-center gap-2 mb-3">
              <div className="h-px flex-1 bg-smoke-600/50" />
              <span className="text-smoke-300 text-sm font-medium">変換結果</span>
              <div className="h-px flex-1 bg-smoke-600/50" />
            </div>
            <div className="bg-smoke-700/40 rounded-xl p-5 border border-amber-500/20">
              <p className="result-text text-smoke-100 text-lg leading-relaxed">
                {result}
              </p>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}

export default TransformCard
