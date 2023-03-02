// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { InstanceState } from './InstanceState';
import type { InstanceUuid } from './InstanceUuid';

export type GameType = Record<Games, {variant: GameVariants}>
export type Games = "MinecraftJava";
export type GameVariants = "Vanilla" | "Forge" | "Fabric" | "Paper";
export interface InstanceInfo {
  uuid: InstanceUuid;
  name: string;
  flavour: string;
  game_type: GameType;
  cmd_args: Array<string>;
  description: string;
  port: number;
  min_ram: number | null;
  max_ram: number | null;
  creation_time: bigint;
  path: string;
  auto_start: boolean;
  restart_on_crash: boolean;
  backup_period: number | null;
  state: InstanceState;
  player_count: number | null;
  max_player_count: number | null;
}
