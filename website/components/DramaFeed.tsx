'use client'

import { motion } from 'framer-motion'

interface DramaEvent {
  id: string
  round: number
  timestamp: string
  player1: {
    address: string
    choice: 'cooperate' | 'snitch'
    result: 'win' | 'lose' | 'neutral'
    amount: string
  }
  player2: {
    address: string
    choice: 'cooperate' | 'snitch'
    result: 'win' | 'lose' | 'neutral'
    amount: string
  }
  outcome: 'mutual_cooperate' | 'one_snitch' | 'mutual_snitch'
}

const mockDrama: DramaEvent[] = [
  {
    id: '1',
    round: 89,
    timestamp: '2 hours ago',
    player1: { address: '0x7f3e...2a1b', choice: 'snitch', result: 'win', amount: '+52,431 $SNITCH' },
    player2: { address: '0x3a8c...9f2d', choice: 'cooperate', result: 'lose', amount: '-52,431 $SNITCH' },
    outcome: 'one_snitch'
  },
  {
    id: '2',
    round: 88,
    timestamp: '26 hours ago',
    player1: { address: '0x9b2f...4e1a', choice: 'cooperate', result: 'neutral', amount: '+3,241 $SNITCH' },
    player2: { address: '0x1c5d...7a3b', choice: 'cooperate', result: 'neutral', amount: '+2,891 $SNITCH' },
    outcome: 'mutual_cooperate'
  },
  {
    id: '3',
    round: 87,
    timestamp: '2 days ago',
    player1: { address: '0x4e8a...1c2f', choice: 'snitch', result: 'lose', amount: '-18,720 $SNITCH' },
    player2: { address: '0x6d9b...3e4a', choice: 'snitch', result: 'lose', amount: '-22,340 $SNITCH' },
    outcome: 'mutual_snitch'
  },
  {
    id: '4',
    round: 86,
    timestamp: '3 days ago',
    player1: { address: '0x2f7c...8b1e', choice: 'cooperate', result: 'lose', amount: '-89,120 $SNITCH' },
    player2: { address: '0x8a3d...5f2c', choice: 'snitch', result: 'win', amount: '+89,120 $SNITCH' },
    outcome: 'one_snitch'
  },
  {
    id: '5',
    round: 85,
    timestamp: '4 days ago',
    player1: { address: '0x5c1e...9a4d', choice: 'cooperate', result: 'neutral', amount: '+5,120 $SNITCH' },
    player2: { address: '0x7b2f...1d6e', choice: 'cooperate', result: 'neutral', amount: '+4,890 $SNITCH' },
    outcome: 'mutual_cooperate'
  }
]

interface DramaFeedProps {
  limit?: number
}

export default function DramaFeed({ limit }: DramaFeedProps) {
  const events = limit ? mockDrama.slice(0, limit) : mockDrama

  const getOutcomeLabel = (outcome: DramaEvent['outcome']) => {
    switch (outcome) {
      case 'mutual_cooperate': return { text: 'MUTUAL TRUST', color: 'text-snitch-green', border: 'border-snitch-green' }
      case 'one_snitch': return { text: 'BETRAYAL', color: 'text-snitch-red', border: 'border-snitch-red' }
      case 'mutual_snitch': return { text: 'MUTUAL DESTRUCTION', color: 'text-snitch-white/50', border: 'border-snitch-white/30' }
    }
  }

  const getChoiceEmoji = (choice: 'cooperate' | 'snitch') => {
    return choice === 'cooperate' ? '🤝' : '🔪'
  }

  const getResultColor = (result: 'win' | 'lose' | 'neutral') => {
    switch (result) {
      case 'win': return 'text-snitch-green'
      case 'lose': return 'text-snitch-red'
      case 'neutral': return 'text-snitch-green'
    }
  }

  return (
    <div className="space-y-6">
      {events.map((event, index) => {
        const outcomeStyle = getOutcomeLabel(event.outcome)
        
        return (
          <motion.div
            key={event.id}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: index * 0.1 }}
            className={`drama-card ${outcomeStyle.border}`}
          >
            {/* Header */}
            <div className="flex items-center justify-between mb-6">
              <div className="flex items-center space-x-4">
                <span className="text-snitch-white/50 text-sm">Round #{event.round}</span>
                <span className={`px-3 py-1 text-xs uppercase tracking-wider ${outcomeStyle.color} border ${outcomeStyle.border}`}>
                  {outcomeStyle.text}
                </span>
              </div>
              <span className="text-snitch-white/30 text-sm">{event.timestamp}</span>
            </div>

            {/* Players */}
            <div className="grid md:grid-cols-2 gap-6">
              {/* Player 1 */}
              <div className="p-4 bg-snitch-black border border-snitch-gray">
                <div className="flex items-center justify-between mb-3">
                  <span className="font-mono text-sm">{event.player1.address}</span>
                  <span className="text-2xl">{getChoiceEmoji(event.player1.choice)}</span>
                </div>
                <div className="flex items-center justify-between">
                  <span className={`uppercase text-sm font-bold ${event.player1.choice === 'snitch' ? 'text-snitch-red' : 'text-snitch-green'}`}>
                    {event.player1.choice}
                  </span>
                  <span className={`font-bold ${getResultColor(event.player1.result)}`}>
                    {event.player1.amount}
                  </span>
                </div>
              </div>

              {/* VS Divider (mobile) */}
              <div className="md:hidden flex items-center justify-center">
                <span className="text-snitch-red font-bold">VS</span>
              </div>

              {/* Player 2 */}
              <div className="p-4 bg-snitch-black border border-snitch-gray">
                <div className="flex items-center justify-between mb-3">
                  <span className="font-mono text-sm">{event.player2.address}</span>
                  <span className="text-2xl">{getChoiceEmoji(event.player2.choice)}</span>
                </div>
                <div className="flex items-center justify-between">
                  <span className={`uppercase text-sm font-bold ${event.player2.choice === 'snitch' ? 'text-snitch-red' : 'text-snitch-green'}`}>
                    {event.player2.choice}
                  </span>
                  <span className={`font-bold ${getResultColor(event.player2.result)}`}>
                    {event.player2.amount}
                  </span>
                </div>
              </div>
            </div>

            {/* View transaction link */}
            <div className="mt-4 text-right">
              <a href="#" className="text-snitch-white/30 hover:text-snitch-red text-sm transition-colors">
                View on Solscan →
              </a>
            </div>
          </motion.div>
        )
      })}
    </div>
  )
}
