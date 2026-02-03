/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  output: 'export',
  basePath: '/snitch-game',
  assetPrefix: '/snitch-game/',
  images: {
    unoptimized: true,
  },
}

module.exports = nextConfig
