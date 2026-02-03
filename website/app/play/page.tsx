'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import WalletButton from '@/components/WalletButton'
import InterrogationUI from '@/components/InterrogationUI'
import Countdown from '@/components/Countdown'

export default function PlayPage() {
  const [isConnected, setIsConnected] = useState(false)
  const [isSelected, setIsSelected] = useState(false) // Simulated state

  return (
    <div className="min-h-screen pt-24 pb-16">
      <div className="max-w-4xl mx-auto px-4">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6 }}
          className="text-center mb-12"
        >
          <h1 className="text-5xl md:text-7xl font-bold mb-4">
            THE <span className="text-snitch-red">GAME</span>
          </h1>
          <p className="text-snitch-white/50 text-lg">
            Connect. Hold. Wait for selection. Choose your fate.
          </p>
        </motion.div>

        {/* Connection state */}
        {!isConnected ? (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.3 }}
            className="text-center"
          >
            {/* Connection prompt */}
            <div className="max-w-lg mx-auto p-8 bg-snitch-darkgray border border-snitch-gray mb-8">
              <div className="text-6xl mb-6">🔒</div>
              <h2 className="text-2xl font-bold mb-4">CONNECT TO PLAY</h2>
              <p className="text-snitch-white/50 mb-8">
                Connect your wallet to check if you&apos;re holding $SNITCH and eligible for the next round.
              </p>
              <WalletButton onConnect={() => setIsConnected(true)} />
            </div>

            {/* Rules */}
            <div className="grid md:grid-cols-3 gap-6 text-left">
              <div className="p-6 bg-snitch-darkgray border border-snitch-gray">
                <div className="text-snitch-red text-3xl mb-4">01</div>
                <h3 className="font-bold mb-2">HOLD $SNITCH</h3>
                <p className="text-snitch-white/50 text-sm">
                  You need at least 1000 $SNITCH to be eligible for selection.
                </p>
              </div>
              <div className="p-6 bg-snitch-darkgray border border-snitch-gray">
                <div className="text-snitch-red text-3xl mb-4">02</div>
                <h3 className="font-bold mb-2">GET SELECTED</h3>
                <p className="text-snitch-white/50 text-sm">
                  Every 24 hours, 2 random holders are chosen for the interrogation.
                </p>
              </div>
              <div className="p-6 bg-snitch-darkgray border border-snitch-gray">
                <div className="text-snitch-red text-3xl mb-4">03</div>
                <h3 className="font-bold mb-2">MAKE YOUR CHOICE</h3>
                <p className="text-snitch-white/50 text-sm">
                  You have 1 hour to decide: COOPERATE or SNITCH. Choose wisely.
                </p>
              </div>
            </div>
          </motion.div>
        ) : isSelected ? (
          /* Player is selected - show interrogation UI */
          <InterrogationUI />
        ) : (
          /* Connected but not selected */
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="text-center"
          >
            {/* Status card */}
            <div className="max-w-lg mx-auto p-8 bg-snitch-darkgray border border-snitch-gray mb-8">
              <div className="flex items-center justify-center space-x-2 mb-6">
                <div className="w-3 h-3 bg-snitch-green rounded-full animate-pulse" />
                <span className="text-snitch-green uppercase tracking-wider text-sm">Connected</span>
              </div>
              
              <div className="mb-6">
                <div className="text-snitch-white/50 text-sm mb-1">YOUR BALANCE</div>
                <div className="text-4xl font-bold">12,450 $SNITCH</div>
              </div>

              <div className="mb-6 p-4 bg-snitch-black border border-snitch-gray">
                <div className="text-snitch-white/50 text-sm mb-1">STATUS</div>
                <div className="text-snitch-green font-bold">ELIGIBLE FOR SELECTION</div>
              </div>

              <div className="text-snitch-white/50 text-sm mb-2">Next selection in</div>
              <Countdown />

              <div className="mt-6 pt-6 border-t border-snitch-gray">
                <div className="text-snitch-white/50 text-sm mb-2">Selection probability</div>
                <div className="text-2xl font-bold text-snitch-red">0.16%</div>
                <div className="text-snitch-white/30 text-xs mt-1">
                  Based on your holdings vs total supply
                </div>
              </div>
            </div>

            {/* Your history */}
            <div className="max-w-lg mx-auto">
              <h3 className="text-xl font-bold mb-4 text-left">YOUR HISTORY</h3>
              <div className="space-y-3">
                <div className="p-4 bg-snitch-darkgray border border-snitch-green flex items-center justify-between">
                  <div>
                    <div className="text-snitch-green font-bold">COOPERATED</div>
                    <div className="text-snitch-white/50 text-sm">Round #67 • vs 0x7f3...2a1</div>
                  </div>
                  <div className="text-right">
                    <div className="text-snitch-green font-bold">+5%</div>
                    <div className="text-snitch-white/50 text-sm">+623 $SNITCH</div>
                  </div>
                </div>
                <div className="p-4 bg-snitch-darkgray border border-snitch-red flex items-center justify-between">
                  <div>
                    <div className="text-snitch-red font-bold">GOT SNITCHED</div>
                    <div className="text-snitch-white/50 text-sm">Round #45 • by 0x3a8...9f2</div>
                  </div>
                  <div className="text-right">
                    <div className="text-snitch-red font-bold">-50%</div>
                    <div className="text-snitch-white/50 text-sm">-8,241 $SNITCH</div>
                  </div>
                </div>
              </div>
            </div>

            {/* Demo button */}
            <div className="mt-12">
              <button
                onClick={() => setIsSelected(true)}
                className="px-6 py-3 border border-snitch-white/20 text-snitch-white/50 hover:border-snitch-red hover:text-snitch-red transition-colors text-sm uppercase tracking-wider"
              >
                [Demo] Simulate Selection
              </button>
            </div>
          </motion.div>
        )}
      </div>
    </div>
  )
}
