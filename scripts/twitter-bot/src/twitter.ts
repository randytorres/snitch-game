import { TwitterApi } from "twitter-api-v2";

type TwitterConfig = {
  appKey: string;
  appSecret: string;
  accessToken: string;
  accessSecret: string;
  bearerToken?: string;
  userId: string;
  dryRun?: boolean;
};

export class TwitterClient {
  private client: TwitterApi;
  private userId: string;
  private dryRun: boolean;

  constructor(config: TwitterConfig) {
    this.client = new TwitterApi({
      appKey: config.appKey,
      appSecret: config.appSecret,
      accessToken: config.accessToken,
      accessSecret: config.accessSecret,
      bearerToken: config.bearerToken,
    });
    this.userId = config.userId;
    this.dryRun = config.dryRun ?? false;
  }

  async postTweet(text: string) {
    if (this.dryRun) {
      console.log("[DRY_RUN] Tweet:\n", text);
      return { id: "dry_run", text };
    }
    return this.client.v2.tweet(text);
  }

  async replyToTweet(text: string, inReplyToId: string) {
    if (this.dryRun) {
      console.log(`[DRY_RUN] Reply to ${inReplyToId}:\n`, text);
      return { id: "dry_run_reply", text };
    }
    return this.client.v2.tweet({
      text,
      reply: { in_reply_to_tweet_id: inReplyToId },
    });
  }

  async getMentions(sinceId?: string) {
    return this.client.v2.userMentionTimeline(this.userId, {
      since_id: sinceId,
      max_results: 50,
      "tweet.fields": ["id", "text", "author_id", "created_at"],
    });
  }
}
