// TODO: Remove expo-crypto dependency
import { randomUUID } from "expo-crypto";
import type { DateString } from "@/types/date_string";

export type CheckListId = string;

export type CheckList = {
  date: DateString;
  id: string;
};

export function newCheckList(props: Omit<CheckList, "id">): CheckList {
  const { date } = props;
  return {
    date,
    id: randomUUID(),
  };
}
