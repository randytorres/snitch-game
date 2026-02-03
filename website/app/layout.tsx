import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: '$SNITCH | The Prisoner\'s Dilemma Token',
  description: 'Trust no one. Every 24 hours, two holders face the ultimate choice: COOPERATE or SNITCH. The game theory token that rewards betrayal.',
  keywords: ['crypto', 'token', 'prisoner dilemma', 'web3', 'solana', 'defi', 'game theory'],
  openGraph: {
    title: '$SNITCH | The Prisoner\'s Dilemma Token',
    description: 'Trust no one. COOPERATE or SNITCH. The choice is yours.',
    type: 'website',
  },
  twitter: {
    card: 'summary_large_image',
    title: '$SNITCH | The Prisoner\'s Dilemma Token',
    description: 'Trust no one. COOPERATE or SNITCH. The choice is yours.',
  },
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className="font-mono antialiased">
        {/* Scanline overlay for CRT effect */}
        <div className="scanline-overlay" />
        
        {/* Interrogation room light effect */}
        <div className="interrogation-light" />
        
        {/* Navigation */}
        <nav className="fixed top-0 left-0 right-0 z-50 bg-snitch-black/90 backdrop-blur-sm border-b border-snitch-gray">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="flex items-center justify-between h-16">
              <a href="/" className="flex items-center space-x-2 group">
                <span className="text-2xl font-bold text-snitch-red flicker-text group-hover:animate-glitch">
                  $SNITCH
                </span>
              </a>
              
              <div className="hidden md:flex items-center space-x-8">
                <a 
                  href="/drama" 
                  className="text-snitch-white/70 hover:text-snitch-red transition-colors uppercase tracking-wider text-sm"
                >
                  Drama Feed
                </a>
                <a 
                  href="/play" 
                  className="text-snitch-white/70 hover:text-snitch-red transition-colors uppercase tracking-wider text-sm"
                >
                  Play
                </a>
                <a 
                  href="#" 
                  className="text-snitch-white/70 hover:text-snitch-red transition-colors uppercase tracking-wider text-sm"
                >
                  Docs
                </a>
              </div>

              <div className="flex items-center space-x-4">
                <a 
                  href="#" 
                  className="text-snitch-white/70 hover:text-snitch-white transition-colors"
                  aria-label="Twitter"
                >
                  <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/>
                  </svg>
                </a>
                <a 
                  href="#" 
                  className="text-snitch-white/70 hover:text-snitch-white transition-colors"
                  aria-label="Telegram"
                >
                  <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm5.562 8.161c-.18 1.897-.962 6.502-1.359 8.627-.168.9-.5 1.201-.82 1.23-.697.064-1.226-.461-1.901-.903-1.056-.693-1.653-1.124-2.678-1.8-1.185-.78-.417-1.21.258-1.91.177-.184 3.247-2.977 3.307-3.23.007-.032.014-.15-.056-.212s-.174-.041-.249-.024c-.106.024-1.793 1.139-5.062 3.345-.479.329-.913.489-1.302.481-.428-.009-1.252-.242-1.865-.442-.751-.244-1.349-.374-1.297-.789.027-.216.325-.437.893-.663 3.498-1.524 5.831-2.529 6.998-3.015 3.333-1.386 4.025-1.627 4.477-1.635.099-.002.321.023.465.141.121.099.154.232.17.325.015.093.034.306.019.472z"/>
                  </svg>
                </a>
              </div>
            </div>
          </div>
        </nav>

        <main className="relative z-10">
          {children}
        </main>

        {/* Footer */}
        <footer className="relative z-10 border-t border-snitch-gray bg-snitch-black py-8">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="flex flex-col md:flex-row items-center justify-between">
              <div className="text-snitch-white/50 text-sm">
                © 2024 $SNITCH. Trust no one.
              </div>
              <div className="flex items-center space-x-6 mt-4 md:mt-0">
                <a href="#" className="text-snitch-white/50 hover:text-snitch-red text-sm transition-colors">
                  Terms
                </a>
                <a href="#" className="text-snitch-white/50 hover:text-snitch-red text-sm transition-colors">
                  Privacy
                </a>
                <a href="#" className="text-snitch-white/50 hover:text-snitch-red text-sm transition-colors">
                  Contract
                </a>
              </div>
            </div>
          </div>
        </footer>
      </body>
    </html>
  )
}
