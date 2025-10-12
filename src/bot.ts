import {
  ChildProcessWithoutNullStreams,
  spawn,
} from "node:child_process";
import { Engine, Key, KeyPress, Room } from "./ty";
import { Types } from "@haelp/teto";

export enum FinesseStyle {
  Human = "human",
  Instant = "instant",
}

export interface BotOptions {
  pps: number;
  vision: number;
  foresight: number;
  can180: boolean;
  finesse: FinesseStyle;
  kicktable: string;
}

export const option_descriptions: Record<keyof BotOptions, string> = {
  pps: "pieces per second during [normal] pace",
  vision: "amount of pieces in the queue to consider",
  foresight:
    "the amount of pieces after [vision] to guess for in order to break ties",
  can180: "whether to do 180s",
  finesse: 'style of placements; either "human" or "instant"',
  kicktable: "which kicktable to use",
};

export class Bot {
  public fps: number = 60;
  public spool!: ChildProcessWithoutNullStreams;
  private buffer = "";
  private resolver: ((s: string) => void) | null = null;

  public options: BotOptions = {
    pps: 4,
    vision: 7,
    foresight: 1,
    can180: true,
    finesse: FinesseStyle.Human,
    kicktable: "srsx",
  }

  public mino_count(c: Engine): number {
    return c.board.state.flat().filter((x) => x !== null).length;
  }

  public constructor() {
    this.reset();
  }

  public async tick(c: Engine): Promise<Types.Game.Tick.Out> {
    const keys: Array<KeyPress> = [];
    return { keys };
  }

  public key_presses(ks: Array<Key>, c: Engine): Array<KeyPress> {
    const keys: Array<KeyPress> = [];
    // keys.push({ frame: c.frame, data: { key: "softDrop", subframe: 0.0 }, type: 'keydown' });
    if (this.options.finesse === FinesseStyle.Human) {
      // if playing at `p` pps then each input should take `fps/pps/n` frames for a piece that needs `n` inputs
      let delta = this.fps / this.options.pps / ks.length;
      for (let i = 0; i < ks.length; i++) {
        const whole = c.frame + Math.floor(i * delta);
        const fract = i * delta - Math.floor(i * delta);
        keys.push({
          frame: whole,
          data: { key: ks[i], subframe: fract },
          type: "keydown",
        });
        keys.push({
          frame: whole,
          data: { key: ks[i], subframe: fract + 0.1 },
          type: "keyup",
        });
      }
    } else if (this.options.finesse === FinesseStyle.Instant) {
      let r_subframe = 0;
      for (const key of ks) {
        keys.push({
          frame: c.frame,
          type: "keydown",
          data: { key, subframe: r_subframe },
        });

        if (key === "softDrop") {
          r_subframe += 0.1;
        }

        keys.push({
          frame: c.frame,
          type: "keyup",
          data: { key, subframe: r_subframe },
        });
      }
    }

    return keys;
  }

  public async reset() {
    this.spool = spawn("./engine/target/release/engine", [this.options.kicktable]);
    // this.spool = spawn("./engine/target/release/engine");
    this.spool.stdout.setEncoding("utf-8");
    this.spool.stderr.setEncoding("utf-8");
    this.spool.stdout.on("data", (data) => {
      this.buffer += data;
      const lines = this.buffer.split("\n");
      if (lines.length > 1) {
        const line = lines[0].trim();
        this.buffer = lines.slice(1).join("\n");
        if (this.resolver) {
          this.resolver(line);
          this.resolver = null;
        }
      }
    });
    this.spool.stderr.on("data", (data) => {
      process.stderr.write(data);
    });
  }

  public async send(input: string): Promise<string> {
    // tracing.debug("send", input);

    return new Promise<string>((resolve) => {
      this.resolver = resolve;
      this.spool.stdin.write(input + "\n", "utf-8");
    });
  }

  public flags(c: Engine): string {
    return [
      this.options.can180 ? "f" : "-",
      "t",
      "-",
    ].join("");
  }



  public async save(): Promise<void> {
    await this.send("ex");
  }
}
