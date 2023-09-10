import type { BottomTabScreenProps } from "@react-navigation/bottom-tabs";
import { StackActions } from "@react-navigation/native";
import React from "react";
import { FlatList, StyleSheet, View } from "react-native";
import { Button } from "react-native-paper";
import { useHistoryScreen } from "@/components/HistoryScreen/hooks/useHistoryScreen";
import type { BottomTabParamList } from "@/types/navigation";

type Props = BottomTabScreenProps<BottomTabParamList, "History">;

export function HistoryScreen({ navigation }: Props): JSX.Element {
  const { dates } = useHistoryScreen();
  return (
    <View style={styles.container}>
      <FlatList
        data={dates}
        keyExtractor={(item) => item}
        renderItem={({ item }) => (
          <Button
            onPress={() => {
              navigation.dispatch(StackActions.push("List"));
            }}
          >
            {item}
          </Button>
        )}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    alignContent: "flex-start",
    alignItems: "stretch",
    display: "flex",
    flexDirection: "column",
    height: "100%",
    justifyContent: "flex-start",
    margin: 0,
    padding: 0,
    width: "100%",
  },
});
