import { Checkbox, List } from "react-native-paper";
import type { Item } from "../types/item";

type Props = {
  checked: boolean;
  item: Item;
  onPress: () => void;
};

export function ListItem({ checked, item, onPress }: Props): JSX.Element {
  return (
    <List.Item
      left={(props) => (
        <Checkbox
          onPress={onPress}
          status={checked ? "checked" : "unchecked"}
          {...props}
        />
      )}
      title={item.name}
    />
  );
}
