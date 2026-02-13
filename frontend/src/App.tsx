import TransformCard from './components/TransformCard'

function App() {
  return (
    <div className="min-h-screen bg-smoke-900 flex flex-col items-center justify-center p-4 relative overflow-hidden">
      {/* Background smoke particles */}
      <div className="absolute inset-0 pointer-events-none opacity-30">
        <div className="smoke-particle" />
        <div className="smoke-particle" />
        <div className="smoke-particle" />
        <div className="smoke-particle" />
        <div className="smoke-particle" />
      </div>

      {/* Header */}
      <h1 className="text-4xl md:text-5xl font-bold text-smoke-100 mb-8 text-center relative z-10 tracking-wide">
        <span className="inline-block animate-fade-in">
          {'\uD83D\uDEAC'} 喫煙者構文変換
        </span>
      </h1>

      {/* Main card */}
      <TransformCard />

      {/* Footer */}
      <p className="mt-8 text-smoke-400 text-sm relative z-10">
        テキストを喫煙者構文に変換します
      </p>
    </div>
  )
}

export default App
