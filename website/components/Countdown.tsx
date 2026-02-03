'use client'

import { useState, useEffect } from 'react'

interface CountdownProps {
  targetDate?: Date
}

export default function Countdown({ targetDate }: CountdownProps) {
  const [timeLeft, setTimeLeft] = useState({
    hours: 0,
    minutes: 0,
    seconds: 0
  })

  useEffect(() => {
    // Default to next midnight UTC if no target provided
    const getNextTarget = () => {
      if (targetDate) return targetDate.getTime()
      const now = new Date()
      const tomorrow = new Date(now)
      tomorrow.setUTCHours(24, 0, 0, 0)
      return tomorrow.getTime()
    }

    const calculateTimeLeft = () => {
      const difference = getNextTarget() - new Date().getTime()
      
      if (difference > 0) {
        setTimeLeft({
          hours: Math.floor((difference / (1000 * 60 * 60)) % 24),
          minutes: Math.floor((difference / 1000 / 60) % 60),
          seconds: Math.floor((difference / 1000) % 60)
        })
      } else {
        setTimeLeft({ hours: 0, minutes: 0, seconds: 0 })
      }
    }

    calculateTimeLeft()
    const timer = setInterval(calculateTimeLeft, 1000)

    return () => clearInterval(timer)
  }, [targetDate])

  const formatNumber = (num: number) => String(num).padStart(2, '0')

  return (
    <div className="flex items-center justify-center space-x-4">
      <div className="countdown-digit">
        <span className="flicker-text">{formatNumber(timeLeft.hours)}</span>
        <div className="text-xs text-snitch-white/50 mt-1 uppercase tracking-wider">Hours</div>
      </div>
      <div className="text-snitch-red text-4xl font-bold animate-pulse">:</div>
      <div className="countdown-digit">
        <span className="flicker-text">{formatNumber(timeLeft.minutes)}</span>
        <div className="text-xs text-snitch-white/50 mt-1 uppercase tracking-wider">Minutes</div>
      </div>
      <div className="text-snitch-red text-4xl font-bold animate-pulse">:</div>
      <div className="countdown-digit">
        <span className="flicker-text">{formatNumber(timeLeft.seconds)}</span>
        <div className="text-xs text-snitch-white/50 mt-1 uppercase tracking-wider">Seconds</div>
      </div>
    </div>
  )
}
