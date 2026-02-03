import "dotenv/config";
import express from "express";
import { TwitterClient } from "./twitter.js";
import { outcomeAnnouncement, dailyRecap, countdownAlert, randomMentionReply } from "./templates.js";
import { scheduleCountdownAlert, startScheduler } from "./scheduler.js";

const requiredEnv = [
  "TWITTER_APP_KEY",
  "TWITTER_APP_SECRET",
  "TWITTER_ACCESS_TOKEN",
  "TWITTER_ACCESS_SECRET",
  "TWITTER_USER_ID",
];

for (const key of requiredEnv) {
  if (!process.env[key]) {
    throw new Error(`Missing env var ${key}`);
  }
}

const twitter = new TwitterClient({
  appKey: process.env.TWITTER_APP_KEY!,
  appSecret: process.env.TWITTER_APP_SECRET!,
  accessToken: process.env.TWITTER_ACCESS_TOKEN!,
  accessSecret: process.env.TWITTER_ACCESS_SECRET!,
  bearerToken: process.env.TWITTER_BEARER_TOKEN,
  userId: process.env.TWITTER_USER_ID!,
  dryRun: process.env.DRY_RUN === "true",
});

const app = express();
app.use(express.json({ limit: "1mb" }));

app.get("/health", (_req, res) => {
  res.json({ ok: true });
});

type InterrogationResolvedPayload = {
  wallet1: string;
  wallet2: string;
  winner: string;
  outcome: "SNITCHED" | "COOPERATED" | string;
  amount: number | string;
  tokenSymbol?: string;
  nextInterrogationIn?: string;
  nextInterrogationAt?: string; // ISO timestamp
};

app.post("/webhook/interrogation", async (req, res) => {
  const payload = req.body as InterrogationResolvedPayload;
  if (!payload?.wallet1 || !payload?.wallet2 || !payload?.winner) {
    res.status(400).json({ error: "Invalid payload" });
    return;
  }

  const tweet = outcomeAnnouncement({
    wallet1: payload.wallet1,
    wallet2: payload.wallet2,
    winner: payload.winner,
    outcome: payload.outcome ?? "SNITCHED",
    amount: payload.amount ?? 0,
    tokenSymbol: payload.tokenSymbol ?? "$SNITCH",
    nextInterrogationIn: payload.nextInterrogationIn,
  });

  await twitter.postTweet(tweet);

  if (payload.nextInterrogationAt) {
    const nextAt = new Date(payload.nextInterrogationAt);
    scheduleCountdownAlert(nextAt, {
      onCountdownAlert: async () => twitter.postTweet(countdownAlert()),
    });
  }

  res.json({ ok: true });
});

const fetchDailyStats = async () => {
  if (!process.env.STATS_ENDPOINT) return null;
  const res = await fetch(process.env.STATS_ENDPOINT);
  if (!res.ok) throw new Error(`Stats endpoint error: ${res.status}`);
  return res.json() as Promise<{
    gamesPlayed: number;
    snitches: number;
    cooperations: number;
    tokensBurned: number | string;
    tokensStolen: number | string;
  }>;
};

startScheduler({
  onDailyRecap: async () => {
    const stats = await fetchDailyStats();
    if (!stats) {
      console.log("No STATS_ENDPOINT configured. Skipping daily recap.");
      return;
    }
    const tweet = dailyRecap(stats);
    await twitter.postTweet(tweet);
  },
  onCountdownAlert: async () => twitter.postTweet(countdownAlert()),
});

let lastMentionId: string | undefined;
const pollMentions = async () => {
  try {
    const mentions = await twitter.getMentions(lastMentionId);
    const tweets = mentions?.data?.data ?? [];
    if (tweets.length === 0) return;

    // Reply oldest-first
    const sorted = [...tweets].sort((a, b) => Number(a.id) - Number(b.id));
    for (const mention of sorted) {
      const reply = randomMentionReply();
      await twitter.replyToTweet(reply, mention.id);
      lastMentionId = mention.id;
    }
  } catch (err) {
    console.error("Mention polling failed", err);
  }
};

const intervalSec = Number(process.env.MENTION_POLL_INTERVAL_SEC ?? 300);
setInterval(pollMentions, intervalSec * 1000);

const port = Number(process.env.PORT ?? 8080);
app.listen(port, () => {
  console.log(`SNITCH bot listening on :${port}`);
});
