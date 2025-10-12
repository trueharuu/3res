import { RoomOptions } from "./ty";

export function check_settings(room: RoomOptions) {
  const fixes = [];
  if (room.boardwidth !== 4) {
    fixes.push("options.boardwidth=4");
  }

  if (room.stock !== 0) {
    fixes.push("options.stock=0");
  }

  if (room.kickset !== "SRS-X") {
    fixes.push("options.kickset=SRS-X");
  }

  if (!room.allow_harddrop) {
    fixes.push("options.allow_harddrop=true");
  }

  return fixes;
}
