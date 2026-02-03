'use client'

import { useState, useEffect } from 'react'
import { motion } from 'framer-motion'
import Countdown from '@/components/Countdown'
import DramaFeed from '@/components/DramaFeed'

export default function Home() {
  const [isLoaded, setIsLoaded] = useState(false)
  
  useEffect(() => {
    setIsLoaded(true)
  }, [])

  return (
    <div className="min-h-screen pt-16">
      {/* Hero Section */}
      <section className="relative min-h-[90vh] flex items-center justify-center overflow-hidden">
        {/* Animated background elements */}
        <div className="absolute inset-0">
          <div className="absolute top-1/4 left-1/4 w-96 h-96 bg-snitch-red/5 rounded-full blur-3xl animate-pulse-slow" />
          <div className="absolute bottom-1/4 right-1/4 w-96 h-96 bg-snitch-red/5 rounded-full blur-3xl animate-pulse-slow delay-1000" />
        </div>

        <div className="relative z-10 max-w-5xl mx-auto px-4 text-center">
          <motion.div
            initial={{ opacity: 0, y: 30 }}
            animate={isLoaded ? { opacity: 1, y: 0 } : {}}
            transition={{ duration: 0.8 }}
          >
            {/* Eyebrow text */}
            <p className="text-snitch-red uppercase tracking-[0.3em] text-sm mb-6 flicker-text">
              The Prisoner&apos;s Dilemma Token
            </p>

            {/* Main title with glitch effect */}
            <h1 
              className="text-7xl md:text-9xl font-black text-snitch-white mb-8 glitch"
              data-text="$SNITCH"
            >
              $SNITCH
            </h1>

            {/* Tagline */}
            <p className="text-xl md:text-2xl text-snitch-white/70 max-w-2xl mx-auto mb-12 leading-relaxed">
              Every 24 hours, two holders are selected.
              <br />
              <span className="text-snitch-red font-bold">COOPERATE</span> or{' '}
              <span className="text-snitch-green font-bold">SNITCH</span>.
              <br />
              <span className="text-snitch-white/50 text-lg">Trust no one.</span>
            </p>

            {/* CTA Buttons */}
            <div className="flex flex-col sm:flex-row items-center justify-center gap-4 mb-16">
              <motion.a
                href="/play"
                className="btn-snitch"
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
              >
                Enter Interrogation
              </motion.a>
              <motion.a
                href="#how-it-works"
                className="px-8 py-4 font-bold uppercase tracking-widest border-2 border-snitch-white/30 text-snitch-white/70 hover:border-snitch-white hover:text-snitch-white transition-all"
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
              >
                Learn More
              </motion.a>
            </div>

            {/* Countdown */}
            <div className="mb-8">
              <p className="text-snitch-white/50 uppercase tracking-wider text-sm mb-4">
                Next Selection In
              </p>
              <Countdown />
            </div>
          </motion.div>
        </div>

        {/* Scroll indicator */}
        <motion.div 
          className="absolute bottom-8 left-1/2 transform -translate-x-1/2"
          animate={{ y: [0, 10, 0] }}
          transition={{ repeat: Infinity, duration: 2 }}
        >
          <svg className="w-6 h-6 text-snitch-white/30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 14l-7 7m0 0l-7-7m7 7V3" />
          </svg>
        </motion.div>
      </section>

      {/* How It Works */}
      <section id="how-it-works" className="py-24 bg-snitch-darkgray">
        <div className="max-w-6xl mx-auto px-4">
          <motion.div
            initial={{ opacity: 0 }}
            whileInView={{ opacity: 1 }}
            transition={{ duration: 0.8 }}
            viewport={{ once: true }}
          >
            <h2 className="text-4xl md:text-5xl font-bold text-center mb-4">
              THE <span className="text-snitch-red">GAME</span>
            </h2>
            <p className="text-snitch-white/50 text-center mb-16 max-w-2xl mx-auto">
              Classic game theory meets DeFi. Every decision has consequences.
            </p>

            {/* Game matrix */}
            <div className="grid md:grid-cols-3 gap-8 mb-16">
              {/* Scenario 1 */}
              <motion.div 
                className="drama-card text-center"
                whileHover={{ y: -5 }}
              >
                <div className="text-snitch-green text-6xl mb-4">🤝</div>
                <h3 className="text-xl font-bold mb-2">BOTH COOPERATE</h3>
                <p className="text-snitch-white/70 mb-4">
                  Mutual trust is rewarded
                </p>
                <div className="text-snitch-green font-bold text-2xl">
                  +5% YIELD EACH
                </div>
              </motion.div>

              {/* Scenario 2 */}
              <motion.div 
                className="drama-card text-center border-snitch-red"
                whileHover={{ y: -5 }}
              >
                <div className="text-snitch-red text-6xl mb-4">🔪</div>
                <h3 className="text-xl font-bold mb-2">ONE SNITCHES</h3>
                <p className="text-snitch-white/70 mb-4">
                  Betrayal pays... for one
                </p>
                <div className="text-snitch-red font-bold text-2xl">
                  SNITCH TAKES 50%
                </div>
                <div className="text-snitch-white/50 text-sm mt-2">
                  of victim&apos;s bag
                </div>
              </motion.div>

              {/* Scenario 3 */}
              <motion.div 
                className="drama-card text-center"
                whileHover={{ y: -5 }}
              >
                <div className="text-snitch-white/50 text-6xl mb-4">💀</div>
                <h3 className="text-xl font-bold mb-2">BOTH SNITCH</h3>
                <p className="text-snitch-white/70 mb-4">
                  Mutual destruction
                </p>
                <div className="text-snitch-white/50 font-bold text-2xl">
                  BOTH LOSE 25%
                </div>
              </motion.div>
            </div>

            {/* Stats */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-center">
              <div className="p-6 border border-snitch-gray">
                <div className="text-3xl font-bold text-snitch-red mb-2">47%</div>
                <div className="text-snitch-white/50 text-sm uppercase tracking-wider">Snitch Rate</div>
              </div>
              <div className="p-6 border border-snitch-gray">
                <div className="text-3xl font-bold text-snitch-green mb-2">$2.4M</div>
                <div className="text-snitch-white/50 text-sm uppercase tracking-wider">Total Snitched</div>
              </div>
              <div className="p-6 border border-snitch-gray">
                <div className="text-3xl font-bold text-snitch-white mb-2">1,247</div>
                <div className="text-snitch-white/50 text-sm uppercase tracking-wider">Holders</div>
              </div>
              <div className="p-6 border border-snitch-gray">
                <div className="text-3xl font-bold text-snitch-white mb-2">89</div>
                <div className="text-snitch-white/50 text-sm uppercase tracking-wider">Rounds Played</div>
              </div>
            </div>
          </motion.div>
        </div>
      </section>

      {/* Recent Drama Preview */}
      <section className="py-24">
        <div className="max-w-6xl mx-auto px-4">
          <div className="flex items-center justify-between mb-12">
            <h2 className="text-4xl md:text-5xl font-bold">
              RECENT <span className="text-snitch-red">DRAMA</span>
            </h2>
            <a 
              href="/drama" 
              className="text-snitch-red hover:text-snitch-white transition-colors uppercase tracking-wider text-sm"
            >
              View All →
            </a>
          </div>
          <DramaFeed limit={3} />
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-24 bg-snitch-darkgray">
        <div className="max-w-4xl mx-auto px-4 text-center">
          <h2 className="text-4xl md:text-6xl font-bold mb-6">
            READY TO <span className="text-snitch-red">PLAY</span>?
          </h2>
          <p className="text-snitch-white/70 text-xl mb-12">
            Connect your wallet. Hold $SNITCH. Wait for your turn.
            <br />
            <span className="text-snitch-white/50">Will you cooperate... or snitch?</span>
          </p>
          <motion.a
            href="/play"
            className="btn-snitch inline-block"
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
          >
            Connect Wallet
          </motion.a>
        </div>
      </section>
    </div>
  )
}
