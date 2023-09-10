// TODO: Remove expo-crypto dependency
import { randomUUID } from "expo-crypto";

export type ItemId = string;

export type Item = {
  id: ItemId;
  name: string;
};

export function newItem(props: Omit<Item, "id">): Item {
  const { name } = props;
  return {
    id: randomUUID(),
    name,
  };
}
