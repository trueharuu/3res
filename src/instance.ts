import { Classes, Client, Types } from "@haelp/teto";
import { Room } from "./ty";
import { check_settings } from "./check";
import { tracing } from "./tracing";
import { Bot, BotOptions, FinesseStyle, option_descriptions } from "./bot";
import { ty } from "./util";

export class Instance {
  private cl!: Client;
  private room!: Room;
  private bot!: Bot;

  public dead: boolean = false;
  public constructor(
    private options: Classes.ClientOptions,
    private code: string,
  ) {
    process.on("SIGINT", async (c) => {
      // tracing.error(`sigint ${this.code}`);
      await this.kill();
    });
  }
  public async spawn() {
    this.cl = await Client.connect(this.options);
    tracing.perf(`join ${this.code}`);
  }

  public async join() {
    this.room = await this.cl.rooms.join(this.code);

    if (check_settings(this.room.options).length === 0) {
      await this.room.switch("player").catch((c) => { });
    }

    this.bot = new Bot(this.room);
    this.cl.on("room.chat", async (c) => {
      await this.onRoomChat(c);
    });

    this.cl.on("client.game.round.start", async (c) => {
      await this.onGameRoundStart(c);
    });

    this.cl.on("client.game.end", async (c) => {
      await this.onGameRoundEnd(c);
    });

    this.cl.on("client.game.abort", async () => {
      await this.onGameAbort();
    });

    this.cl.on("room.player.remove", async () => {
      const humans = this.room.players.filter((x) => !x.bot);
      if (humans.length === 0) {
        await this.kill();
      }
    });

    this.cl.on("room.update.bracket", async (c) => {
      if (c.uid === this.cl.user.id && c.bracket === "player") {
        const set = check_settings(this.room.options);
        if (set.length > 0) {
          await this.room.switch("spectator");
          await this.room.chat("something is bad! paste the following to fix:");
          await this.room.chat(`/set ${set.join(";")}`);
        }
      }
    });

    this.cl.on("room.update", async (c) => {
      if (check_settings(c.options).length > 0) {
        await this.room.switch("spectator");
      }
    });

    this.cl.on("client.game.start", async (c) => {
      await this.room.chat("glhf");
      await this.bot.save();
      await this.bot.reset();
    });
  }

  private lock: boolean = false;
  public async kill() {
    this.bot.spool.kill();
    await this.room.chat(":crying:");
    await this.room.leave();

    tracing.error(tracing.tag(this.code), "was killed");
  }

  public async onRoomChat(c: Types.Events.in.all["room.chat"]) {
    if (c.system || c.user.role === "bot") {
      return;
    }

    if (!c.content.startsWith(process.env.PREFIX!)) {
      return;
    }

    const argv = c.content
      .slice(1)
      .split(" ")
      .map((x) => x.toLowerCase());

    if (argv[0] === "help") {
      return await this.sendHelp(argv[1]);
    }

    if (argv[0] === "settings") {
      return await this.sendSettings();
    }

    if (argv[0] === "host") {
      // can take host IF
      // the bot IS host AND (you are the bot owner OR the room creator OR the creator is not there)
      if (!this.room.isHost) {
        return await this.room.chat("no! (i'm not host)");
      }

      if (
        this.room.creator === c.user._id ||
        process.env.HOSTS?.split(",").includes(c.user._id) ||
        !this.room.players.some((x) => x._id === this.room.creator)
      ) {
        try {
          await this.room.transferHost(c.user._id);
        } catch {
          return await this.room.chat("no! (failed)");
        }
        return await this.room.chat("ok");
      }
    }

    if (argv[0] === "who") {
      const usr = argv[1];
      if (!usr) {
        return await this.room.chat("no! (missing user)");
      }

      try {
        const u = await this.cl.api.users.get({ username: usr });
        const auth =
          u._id === this.cl.user.id
            ? "bot"
            : process.env.HOSTS?.split(",").includes(u._id)
              ? "admin"
              : this.room.creator === u._id
                ? "roomcreator"
                : this.room.owner === u._id
                  ? "host"
                  : this.room.players.some((x) => x._id === u._id)
                    ? "player"
                    : "nothing";
        return await this.room.chat(
          `${u.username}\n| id = ${u._id}\n| auth = ${auth}`,
        );
      } catch {
        return await this.room.chat("no! (failed)");
      }
    }

    if (
      this.room.owner !== c.user._id &&
      !process.env.HOSTS?.split(",").includes(c.user._id)
    ) {
      return await this.room.chat("no! (unauthorized)");
    }

    if (argv[0] === 'gen') {
      await this.bot.populate();
    }

    if (argv[0] === "pps") {
      const n = Number(argv[1]);
      if (Number.isNaN(n)) {
        return await this.room.chat("no! (not a number)");
      }

      if (n > 30 || n <= 0) {
        return await this.room.chat("no! (must be 0 < pps <= 30)");
      }

      await this.room.chat(`ok pps=${n}`);

      this.bot.options.pps = n;
    }

    if (argv[0] === "vision") {
      const n = Number(argv[1]);
      if (Number.isNaN(n)) {
        return await this.room.chat("no! (not a number)");
      }

      if (n > 35 || n < 2) {
        return await this.room.chat("no! (must be 2 <= vision <= 35)");
      }

      await this.room.chat(`ok vision=${n}`);

      this.bot.options.vision = n;
    }

    if (argv[0] === "n") {
      const n = Number(argv[1]);
      if (Number.isNaN(n)) {
        return await this.room.chat("no! (not a number)");
      }

      if (n > 7 || n < 0) {
        return await this.room.chat("no! (must be 0 <= n <= 7)");
      }

      await this.room.chat(`ok n=${n}`);

      this.bot.options.n = n;
    }

    if (argv[0] === "can180") {
      if (argv[1] === "true") {
        this.bot.options.can180 = true;
      } else if (argv[1] === "false") {
        this.bot.options.can180 = false;
      } else {
        return await this.room.chat(
          'no! (can180 must be one of "true" | "false")',
        );
      }
    }

    if (argv[0] === "finesse") {
      if (argv[1] === "human") {
        this.bot.options.finesse = FinesseStyle.Human;
        await this.room.chat(`ok finesse=${argv[1]}`);
      } else if (argv[1] === "instant") {
        this.bot.options.finesse = FinesseStyle.Instant;
        await this.room.chat(`ok finesse=${argv[1]}`);
      } else {
        return await this.room.chat(
          'no! (finesse must be one of "human", "instant")',
        );
      }
    }
  }

  public async sendHelp(c?: string) {
    if (c) {
      if (c === "help") {
        return await this.room.chat("it's this one");
      }

      if (c in this.bot.options) {
        return await this.room.chat(
          `${c}:\n${option_descriptions[c as keyof typeof option_descriptions]}`,
        );
      }

      return await this.room.chat("no! (not an option)");
    }

    return await this.room.chat(
      `available commands:\n${ty.keys(this.bot.options).join(", ")}\n\njoin 4w lounge:\nhttps://discord.gg/7SnE8xwMMU`,
    );
  }

  public async paceWarning() {
    return await this.room.chat(
      "hey! this setting does nothing without [pace]!",
    );
  }

  public async sendSettings() {
    const keys = Object.keys(this.bot.options);
    const values = Object.values(this.bot.options);

    const longest_k = Math.max(...keys.map((x) => x.length));
    const longest_v = Math.max(...values.map((x) => String(x).length));

    return await this.room.chat(
      "key".padStart(longest_k, " ") +
      " | " +
      "value".padEnd(longest_v, " ") +
      "\n" +
      "-".repeat(longest_k) +
      "---" +
      "-".repeat(longest_v) +
      "\n" +
      keys
        .map(
          (x) =>
            x.padStart(longest_k, " ") +
            " | " +
            this.bot.options[x as keyof typeof this.bot.options],
        )
        .join("\n"),
    );
  }

  public async onGameRoundStart([
    tick,
  ]: Types.Events.in.all["client.game.round.start"]) {
    tick(async (c) => {
      return await this.bot.tick(c.engine);
    });
  }

  public async onGameRoundEnd(c: Types.Events.in.all["client.game.end"]) {
    await this.bot.save();
    await this.bot.reset();

    if (this.room.players.some((x) => x._id === this.cl.user.id)) {
      const won = c.players.filter((x) => x.won);
      if (won.length) {
        if (won.some((x) => x.id === this.cl.user.id)) {
          return await this.room.chat(":happy:");
        } else {
          return await this.room.chat(":sad:");
        }
      }
    }
  }

  public async onGameAbort() {
    return await this.room.chat(":stare:");
  }
}
