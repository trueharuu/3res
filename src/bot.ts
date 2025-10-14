import {
  ChildProcessWithoutNullStreams,
  spawn,
} from "node:child_process";
import { Engine, Key, KeyPress, Room } from "./ty";
import { Types } from "@haelp/teto";
import { tracing } from "./tracing";

export enum FinesseStyle {
  Human = "human",
  Instant = "instant",
}

export interface BotOptions {
  pps: number;
  vision: number;
  n: number;
  can180: boolean;
  finesse: FinesseStyle;
  kicktable: string;
}

export const option_descriptions: Record<keyof BotOptions, string> = {
  pps: "pieces per second during [normal] pace",
  vision: "amount of pieces in the queue to consider",
  n:
    "the tallest PC we are able to perform",
  can180: "whether to do 180s",
  finesse: 'style of placements; either "human" or "instant"',
  kicktable: "which kicktable to use",
};

export class Bot {
  // private room!: Room;
  public fps: number = 60;
  public spool!: ChildProcessWithoutNullStreams;
  private buffer = "";
  private resolver: ((s: string) => void) | null = null;

  public options: BotOptions = {
    pps: 4,
    vision: 7,
    n: 6,
    can180: true,
    finesse: FinesseStyle.Human,
    kicktable: "srsx",
  }

  public mino_count(c: Engine): number {
    return c.board.state.flat().filter((x) => x !== null).length;
  }

  public constructor(private room: Room) {
    this.reset();
  }

  public piece_queue: Array<[string, Array<Key>]> = [];
  private acc: number = 0;
  public async tick(c: Engine): Promise<Types.Game.Tick.Out> {
    // if (c.frame === 0) {
    // await this.populate(c);
    // }
    this.acc += this.options.pps / this.fps;
    const keys: Array<KeyPress> = [];
    while (this.acc >= 1) {
      tracing.error(`${c.held?.toUpperCase() || ''}${c.falling.symbol.toUpperCase()}${c.queue.value.join('').toUpperCase()}`)
      if (this.piece_queue.length === 0) {
        const pq = await this.regenerate(c);
        if (this.dead) {
          if (!this.announced) {
            this.announced = true;
            await this.room.chat(':stare:');
          }

          return { keys: [] }
        }
        tracing.warn(`regenerating ${pq.map(x => x[0])}`);
        this.piece_queue = pq;
      }

      const next = this.piece_queue.shift()!;
      let [t, ks] = next;
      tracing.info(`placing ${t} (${c.falling.symbol} active, ${c.held} hold, ${c.queue.at(0)} next)`);

      if (t.toLowerCase() !== c.falling.symbol) {
        ks.unshift('hold');
      }

      ks.push('hardDrop');
      keys.push(...this.key_presses(ks, c));

      this.acc -= 1;
    }

    return { keys };
  }

  public async populate() {
    this.send(`pcp ${this.flags()} ${this.options.vision} ${this.options.n}`);
  }

  private dead: boolean = false;
  private announced: boolean = false;
  public async regenerate(c: Engine): Promise<Array<[string, Array<Key>]>> {
    const queue = ((c.held || '') + c.falling.symbol + c.queue.value.join('')).toUpperCase();

    const resp = await this.send(`pcr ${this.flags()} ${queue.slice(0, this.options.vision)} ${this.options.n}`);

    if (resp === '!') {
      this.dead = true;
      return [];
    }
    return resp.split(' ').map(x => {
      // console.log('part', x);
      let [piece, f] = x.slice(x.indexOf('(') + 1, x.indexOf(')')).split(':');
      let keys = f.split(',').filter(x => x !== '');
      if (keys.length === 0) {
        keys.unshift('softDrop');
      }
      return [piece, keys] as [string, Array<Key>];
    });
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
    this.announced = false;
    this.dead = false;
    this.piece_queue = [];
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

      tracing.debug("recv", data);
    });
    this.spool.stderr.on("data", (data) => {
      process.stderr.write(data);
    });
  }

  public async send(input: string): Promise<string> {
    tracing.debug("send", input);

    return new Promise<string>((resolve) => {
      this.resolver = resolve;
      this.spool.stdin.write(input + "\n", "utf-8");
    });
  }

  public flags(): string {
    return [
      this.options.can180 ? "f" : "-",
      "t",
      'h',
      "u",
    ].join("");
  }



  public async save(): Promise<void> {
    await this.send("ex");
  }
}
