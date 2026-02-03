'use client'

import { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'

type GameChoice = 'cooperate' | 'snitch' | null
type GamePhase = 'choosing' | 'waiting' | 'reveal' | 'result'

export default function InterrogationUI() {
  const [phase, setPhase] = useState<GamePhase>('choosing')
  const [myChoice, setMyChoice] = useState<GameChoice>(null)
  const [opponentChoice, setOpponentChoice] = useState<GameChoice>(null)
  const [timeLeft, setTimeLeft] = useState(59 * 60 + 47) // 59:47

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60)
    const secs = seconds % 60
    return `${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}`
  }

  const handleChoice = async (choice: GameChoice) => {
    setMyChoice(choice)
    setPhase('waiting')
    
    // Simulate opponent choosing
    await new Promise(resolve => setTimeout(resolve, 3000))
    
    // Random opponent choice for demo
    const oppChoice: GameChoice = Math.random() > 0.5 ? 'snitch' : 'cooperate'
    setOpponentChoice(oppChoice)
    setPhase('reveal')
    
    // Show result after reveal animation
    await new Promise(resolve => setTimeout(resolve, 2000))
    setPhase('result')
  }

  const getOutcome = () => {
    if (myChoice === 'cooperate' && opponentChoice === 'cooperate') {
      return { type: 'mutual_cooperate', myResult: '+5%', text: 'MUTUAL TRUST', color: 'text-snitch-green' }
    } else if (myChoice === 'snitch' && opponentChoice === 'cooperate') {
      return { type: 'i_snitched', myResult: '+50%', text: 'YOU SNITCHED', color: 'text-snitch-red' }
    } else if (myChoice === 'cooperate' && opponentChoice === 'snitch') {
      return { type: 'got_snitched', myResult: '-50%', text: 'YOU GOT SNITCHED', color: 'text-snitch-red' }
    } else {
      return { type: 'mutual_snitch', myResult: '-25%', text: 'MUTUAL DESTRUCTION', color: 'text-snitch-white/50' }
    }
  }

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      className="max-w-2xl mx-auto"
    >
      {/* Alert banner */}
      <motion.div
        className="mb-8 p-4 bg-snitch-red/20 border-2 border-snitch-red pulse-border text-center"
        animate={{ opacity: [1, 0.8, 1] }}
        transition={{ repeat: Infinity, duration: 2 }}
      >
        <div className="text-snitch-red font-bold text-lg mb-1">⚠️ YOU HAVE BEEN SELECTED ⚠️</div>
        <div className="text-snitch-white/70 text-sm">Round #90 • Your opponent is waiting</div>
      </motion.div>

      {/* Main interrogation box */}
      <div className="bg-snitch-darkgray border border-snitch-gray p-8">
        {/* Timer */}
        <div className="text-center mb-8">
          <div className="text-snitch-white/50 text-sm uppercase tracking-wider mb-2">Time Remaining</div>
          <div className="text-5xl font-bold text-snitch-red flicker-text">{formatTime(timeLeft)}</div>
        </div>

        {/* Opponent info */}
        <div className="p-4 bg-snitch-black border border-snitch-gray mb-8">
          <div className="flex items-center justify-between">
            <div>
              <div className="text-snitch-white/50 text-sm mb-1">YOUR OPPONENT</div>
              <div className="font-mono">0x4e8a...1c2f</div>
            </div>
            <div className="text-right">
              <div className="text-snitch-white/50 text-sm mb-1">THEIR BAG</div>
              <div className="font-bold">87,420 $SNITCH</div>
            </div>
          </div>
        </div>

        <AnimatePresence mode="wait">
          {phase === 'choosing' && (
            <motion.div
              key="choosing"
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -20 }}
            >
              <div className="text-center mb-8">
                <h2 className="text-2xl font-bold mb-2">MAKE YOUR CHOICE</h2>
                <p className="text-snitch-white/50">This decision cannot be undone.</p>
              </div>

              <div className="grid grid-cols-2 gap-6">
                <motion.button
                  className="btn-cooperate flex flex-col items-center py-8"
                  whileHover={{ scale: 1.02 }}
                  whileTap={{ scale: 0.98 }}
                  onClick={() => handleChoice('cooperate')}
                >
                  <span className="text-5xl mb-4">🤝</span>
                  <span className="text-xl font-bold">COOPERATE</span>
                  <span className="text-sm mt-2 opacity-70">Trust your opponent</span>
                </motion.button>

                <motion.button
                  className="btn-snitch flex flex-col items-center py-8"
                  whileHover={{ scale: 1.02 }}
                  whileTap={{ scale: 0.98 }}
                  onClick={() => handleChoice('snitch')}
                >
                  <span className="text-5xl mb-4">🔪</span>
                  <span className="text-xl font-bold">SNITCH</span>
                  <span className="text-sm mt-2 opacity-70">Betray for profit</span>
                </motion.button>
              </div>
            </motion.div>
          )}

          {phase === 'waiting' && (
            <motion.div
              key="waiting"
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -20 }}
              className="text-center py-12"
            >
              <div className="text-6xl mb-6">{myChoice === 'cooperate' ? '🤝' : '🔪'}</div>
              <div className="text-xl font-bold mb-4">
                You chose to <span className={myChoice === 'cooperate' ? 'text-snitch-green' : 'text-snitch-red'}>
                  {myChoice?.toUpperCase()}
                </span>
              </div>
              <div className="flex items-center justify-center space-x-2 text-snitch-white/50">
                <svg className="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                <span>Waiting for opponent...</span>
              </div>
            </motion.div>
          )}

          {phase === 'reveal' && (
            <motion.div
              key="reveal"
              initial={{ opacity: 0, scale: 0.8 }}
              animate={{ opacity: 1, scale: 1 }}
              exit={{ opacity: 0 }}
              className="text-center py-12"
            >
              <motion.div
                className="text-8xl mb-6"
                animate={{ rotate: [0, -10, 10, -10, 0] }}
                transition={{ duration: 0.5 }}
              >
                {opponentChoice === 'cooperate' ? '🤝' : '🔪'}
              </motion.div>
              <div className="text-2xl font-bold">
                Opponent chose to{' '}
                <span className={opponentChoice === 'cooperate' ? 'text-snitch-green' : 'text-snitch-red'}>
                  {opponentChoice?.toUpperCase()}
                </span>
              </div>
            </motion.div>
          )}

          {phase === 'result' && (
            <motion.div
              key="result"
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              className="text-center py-8"
            >
              {(() => {
                const outcome = getOutcome()
                return (
                  <>
                    <motion.div
                      className={`text-4xl md:text-5xl font-bold mb-4 ${outcome.color}`}
                      initial={{ scale: 0 }}
                      animate={{ scale: 1 }}
                      transition={{ type: 'spring', stiffness: 200 }}
                    >
                      {outcome.text}
                    </motion.div>
                    
                    <div className="grid grid-cols-2 gap-8 my-8">
                      <div className="p-6 bg-snitch-black border border-snitch-gray">
                        <div className="text-snitch-white/50 text-sm mb-2">YOU</div>
                        <div className="text-4xl mb-2">{myChoice === 'cooperate' ? '🤝' : '🔪'}</div>
                        <div className={`text-2xl font-bold ${outcome.myResult.startsWith('+') ? 'text-snitch-green' : 'text-snitch-red'}`}>
                          {outcome.myResult}
                        </div>
                      </div>
                      <div className="p-6 bg-snitch-black border border-snitch-gray">
                        <div className="text-snitch-white/50 text-sm mb-2">OPPONENT</div>
                        <div className="text-4xl mb-2">{opponentChoice === 'cooperate' ? '🤝' : '🔪'}</div>
                        <div className={`text-2xl font-bold ${
                          (myChoice === 'snitch' && opponentChoice === 'cooperate') ? 'text-snitch-red' :
                          (myChoice === 'cooperate' && opponentChoice === 'snitch') ? 'text-snitch-green' :
                          outcome.myResult.startsWith('+') ? 'text-snitch-green' : 'text-snitch-red'
                        }`}>
                          {myChoice === 'snitch' && opponentChoice === 'cooperate' ? '-50%' :
                           myChoice === 'cooperate' && opponentChoice === 'snitch' ? '+50%' :
                           outcome.myResult}
                        </div>
                      </div>
                    </div>

                    <motion.a
                      href="/drama"
                      className="inline-block px-8 py-4 border-2 border-snitch-gray text-snitch-white hover:border-snitch-red transition-colors uppercase tracking-wider"
                      whileHover={{ scale: 1.02 }}
                    >
                      View in Drama Feed
                    </motion.a>
                  </>
                )
              })()}
            </motion.div>
          )}
        </AnimatePresence>
      </div>

      {/* Game theory reminder */}
      <div className="mt-8 p-4 bg-snitch-darkgray/50 border border-snitch-gray/50">
        <div className="text-snitch-white/30 text-sm text-center">
          <strong>Remember:</strong> Both cooperate = +5% each • One snitches = snitch takes 50% • Both snitch = both lose 25%
        </div>
      </div>
    </motion.div>
  )
}
