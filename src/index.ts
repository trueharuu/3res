import { Client } from "@haelp/teto";
import "dotenv/config.js";
import { tracing } from "./tracing";
import { Instance } from "./instance";
import { Engine, EngineInitializeParams, randomSeed } from "@haelp/teto/engine";
import { Bot } from "./bot";
import { writeFileSync } from "fs";
import { KeyPress } from "./ty";
// import { displayBoard, getNextBoards, hashBoard, unhashBoard } from "./usm";

// console.log(process.env)
// tracing.debug(tracing.level);
process.on("unhandledRejection", (c) => {
  tracing.error(c);
});
process.on("uncaughtException", (c) => {
  tracing.error(c);
});

// import * as solver_lib from "./usm/solver_lib";
(async () => {
  // tracing.info(process.env.TOKEN);
  const handling = {
    arr: 0,
    cancel: false,
    das: 1,
    dcd: 0,
    ihs: "off",
    irs: "off",
    may20g: false,
    safelock: true,
    sdf: 41,
  } as const;
  const login = {
    token: process.env.TOKEN!,
    ribbon: { codec: "candor", verbose: false },
    handling,
  } as const;
  // tracing.info("test");

  tracing.perf("init");
  const master = await Client.connect(login);

  tracing.perf("init");

  master.on("social.relation.add", async (c) => {
    await master.social.friend(c._id).catch(tracing.safe);
  });

  // tracing.info(master.social.friends.map((x) => `${x.username} ${x.id}`));

  const is: Array<Instance> = [];

  master.on("social.invite", async (c) => {
    tracing.perf(`join ${c.roomid}`);
    const instance = new Instance(login, c.roomid);
    await instance.spawn();
    await instance.join();
    is.push(instance);
  });

  process.on("SIGINT", async () => {
    await master.destroy();
    setTimeout(() => {
      tracing.fatal("max timeout hit");
    }, 1000);
  });

  // solo stuff
  // const seed = randomSeed();
  // const date = new Date();
  // const ttc = {
  //   b2b: { chaining: true, charging: false },
  //   board: { width: 4, height: 8, buffer: 20 },
  //   garbage: {
  //     cap: { absolute: 0, increase: 0, max: 40, value: 8, marginTime: 0 },
  //     boardWidth: 4,
  //     garbage: {
  //       speed: 20,
  //       holeSize: 1,
  //     },
  //     messiness: {
  //       change: 1,
  //       nosame: false,
  //       timeout: 0,
  //       within: 0,
  //       center: false,
  //     },
  //     multiplier: {
  //       value: 1,
  //       increase: 0.008,
  //       marginTime: 10800,
  //     },
  //     bombs: false,
  //     specialBonus: false,
  //     openerPhase: 0,
  //     seed,
  //     rounding: "down",
  //   },
  //   gravity: {
  //     value: 0.02,
  //     increase: 0,
  //     marginTime: 0,
  //   },
  //   handling,
  //   kickTable: "SRS-X",
  //   misc: {
  //     allowed: { hardDrop: true, spin180: true, hold: true },
  //     infiniteHold: false,
  //     movement: {
  //       infinite: false,
  //       lockResets: 15,
  //       lockTime: 30,
  //       may20G: true,
  //     },
  //     username: "6res",
  //     date,
  //   },
  //   multiplayer: {
  //     opponents: [],
  //     passthrough: "zero",
  //   },
  //   options: {
  //     comboTable: "multiplier",
  //     garbageBlocking: "combo blocking",
  //     clutch: true,
  //     garbageTargetBonus: "none",
  //     spinBonuses: "handheld",
  //   },
  //   pc: {
  //     b2b: 0,
  //     garbage: 0,
  //   },
  //   queue: {
  //     minLength: 10,
  //     seed,
  //     type: "7-bag",
  //   },
  // } as EngineInitializeParams;

  // let engine = new Engine(ttc);
  // const bot = new Bot();
  // bot.options.vision = 14;
  // bot.options.pps = 1;
  // bot.options.upstack = false;

  // const ttr: Array<KeyPress> = [];

  // let buf: Array<KeyPress> = [];
  // while (!engine.toppedOut && engine.frame < 2 * 60 * bot.fps) {
  //   tracing.info(`ticking ${engine.frame}\n${engine.text}`);
  //   if (buf.length > 0) {
  //     // while (buf[0].frame === engine.frame) {
  //       const pop_len = buf.findIndex((c) => c.frame !== buf[0].frame);
  //       const keys = buf.splice(0, pop_len);
  //       engine.tick(keys);
  //     // }
  //   } else {
  //     const keys = await bot.tick(engine);
  //     buf = keys.keys!;
  //   }
  // }

  // const obj = {
  //   id: null,
  //   gamemode: "custom",
  //   ts: date.toISOString(),
  //   users: [{ id: master.user.id, username: master.user.username }],
  //   replay: {
  //     frames: ttr.length,
  //     events: [
  //       { frame: 0, type: "start", data: {} },
  //       ...ttr,
  //       {
  //         frame: ttr.length,
  //         type: "end",
  //         data: {},
  //       },
  //     ],
  //   },
  // };

  // writeFileSync("solo.ttr", JSON.stringify(obj));
})();
