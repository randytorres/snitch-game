export type OutcomeTemplateInput = {
  wallet1: string;
  wallet2: string;
  winner: string; // wallet handle
  outcome: "SNITCHED" | "COOPERATED" | string;
  amount: number | string;
  tokenSymbol?: string;
  nextInterrogationIn?: string; // e.g. 23:42:19
};

export type DailyStatsInput = {
  gamesPlayed: number;
  snitches: number;
  cooperations: number;
  tokensBurned: number | string;
  tokensStolen: number | string;
};

export const outcomeAnnouncement = (input: OutcomeTemplateInput) => {
  const token = input.tokenSymbol ?? "$SNITCH";
  return [
    "🚨 INTERROGATION RESOLVED 🚨",
    "",
    `${input.wallet1} vs ${input.wallet2}`,
    "",
    `Result: [${input.winner}] ${input.outcome} 🗡️`,
    "",
    `💀 ${input.wallet2} lost ${input.amount} ${token}`,
    `🤑 ${input.wallet1} gained ${input.amount} ${token}`,
    "",
    `Next interrogation in: ${input.nextInterrogationIn ?? "TBD"}`,
    "",
    "Play: snitch.fun",
  ].join("\n");
};

export const dailyRecap = (input: DailyStatsInput) => {
  const total = input.snitches + input.cooperations;
  const snitchPct = total ? Math.round((input.snitches / total) * 100) : 0;
  const coopPct = total ? Math.round((input.cooperations / total) * 100) : 0;
  return [
    "📊 $SNITCH Daily Recap",
    "",
    `Games played: ${input.gamesPlayed}`,
    `Total snitches: ${input.snitches} (${snitchPct}%)`,
    `Total cooperations: ${input.cooperations} (${coopPct}%)`,
    `Tokens burned: ${input.tokensBurned}`,
    `Tokens stolen: ${input.tokensStolen}`,
    "",
    "The trust issues continue...",
  ].join("\n");
};

export const countdownAlert = () => {
  return [
    "⏰ NEW INTERROGATION IN 1 HOUR",
    "",
    "Two wallets will be selected.",
    "One might betray the other.",
    "",
    "Will you be chosen? 👀",
  ].join("\n");
};

const memeReplies = [
  "You rang? 🗡️",
  "Snitches get… tweets. 😈",
  "Trust issues intensify. 👀",
  "No honor among wallets. 💀",
  "We all have a price… what's yours? 🤑",
];

export const randomMentionReply = () => {
  return memeReplies[Math.floor(Math.random() * memeReplies.length)];
};
