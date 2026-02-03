'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'

interface WalletButtonProps {
  onConnect?: () => void
}

export default function WalletButton({ onConnect }: WalletButtonProps) {
  const [isConnecting, setIsConnecting] = useState(false)
  const [isConnected, setIsConnected] = useState(false)
  const [address, setAddress] = useState('')

  const handleConnect = async () => {
    setIsConnecting(true)
    
    // Simulate connection delay
    await new Promise(resolve => setTimeout(resolve, 1500))
    
    // Simulated wallet address
    const mockAddress = '0x7f3e...2a1b'
    setAddress(mockAddress)
    setIsConnected(true)
    setIsConnecting(false)
    
    onConnect?.()
  }

  const handleDisconnect = () => {
    setIsConnected(false)
    setAddress('')
  }

  if (isConnected) {
    return (
      <motion.button
        className="group flex items-center space-x-3 px-6 py-3 bg-snitch-darkgray border border-snitch-gray hover:border-snitch-red transition-colors"
        onClick={handleDisconnect}
        whileHover={{ scale: 1.02 }}
        whileTap={{ scale: 0.98 }}
      >
        <div className="w-2 h-2 bg-snitch-green rounded-full animate-pulse" />
        <span className="text-snitch-white font-mono">{address}</span>
        <span className="text-snitch-white/50 group-hover:text-snitch-red transition-colors text-sm">
          ✕
        </span>
      </motion.button>
    )
  }

  return (
    <motion.button
      className="btn-snitch flex items-center justify-center space-x-3 min-w-[200px]"
      onClick={handleConnect}
      disabled={isConnecting}
      whileHover={{ scale: isConnecting ? 1 : 1.05 }}
      whileTap={{ scale: isConnecting ? 1 : 0.95 }}
    >
      {isConnecting ? (
        <>
          <svg className="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          <span>Connecting...</span>
        </>
      ) : (
        <>
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1" />
          </svg>
          <span>Connect Wallet</span>
        </>
      )}
    </motion.button>
  )
}
