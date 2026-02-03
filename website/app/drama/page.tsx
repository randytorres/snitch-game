'use client'

import { motion } from 'framer-motion'
import DramaFeed from '@/components/DramaFeed'

export default function DramaPage() {
  return (
    <div className="min-h-screen pt-24 pb-16">
      <div className="max-w-6xl mx-auto px-4">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6 }}
          className="mb-12"
        >
          <h1 className="text-5xl md:text-7xl font-bold mb-4">
            DRAMA <span className="text-snitch-red">FEED</span>
          </h1>
          <p className="text-snitch-white/50 text-lg max-w-2xl">
            Every round tells a story. Betrayals, alliances, and the cold mathematics of game theory.
            Watch history unfold.
          </p>
        </motion.div>

        {/* Filters */}
        <div className="flex flex-wrap gap-4 mb-8">
          <button className="px-4 py-2 bg-snitch-red text-snitch-white text-sm uppercase tracking-wider">
            All
          </button>
          <button className="px-4 py-2 border border-snitch-gray text-snitch-white/70 hover:border-snitch-red hover:text-snitch-red text-sm uppercase tracking-wider transition-colors">
            Snitches Only
          </button>
          <button className="px-4 py-2 border border-snitch-gray text-snitch-white/70 hover:border-snitch-green hover:text-snitch-green text-sm uppercase tracking-wider transition-colors">
            Cooperations
          </button>
          <button className="px-4 py-2 border border-snitch-gray text-snitch-white/70 hover:border-snitch-white hover:text-snitch-white text-sm uppercase tracking-wider transition-colors">
            Mutual Destruction
          </button>
        </div>

        {/* Stats bar */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-12 p-4 bg-snitch-darkgray border border-snitch-gray">
          <div className="text-center">
            <div className="text-2xl font-bold text-snitch-red">47%</div>
            <div className="text-snitch-white/50 text-xs uppercase tracking-wider">Snitch Rate</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-snitch-green">38%</div>
            <div className="text-snitch-white/50 text-xs uppercase tracking-wider">Cooperation Rate</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-snitch-white/50">15%</div>
            <div className="text-snitch-white/50 text-xs uppercase tracking-wider">Mutual Snitch</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-snitch-white">89</div>
            <div className="text-snitch-white/50 text-xs uppercase tracking-wider">Total Rounds</div>
          </div>
        </div>

        {/* Drama Feed */}
        <DramaFeed />

        {/* Load More */}
        <div className="text-center mt-12">
          <motion.button
            className="px-8 py-4 border-2 border-snitch-gray text-snitch-white/70 hover:border-snitch-red hover:text-snitch-red uppercase tracking-wider transition-all"
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
          >
            Load More Drama
          </motion.button>
        </div>
      </div>
    </div>
  )
}
