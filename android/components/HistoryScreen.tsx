import type { BottomTabScreenProps } from "@react-navigation/bottom-tabs";
import { StackActions } from "@react-navigation/native";
import React from "react";
import { StyleSheet, View } from "react-native";
import { Button } from "react-native-paper";
import type { BottomTabParamList } from "../types/navigation";

type Props = BottomTabScreenProps<BottomTabParamList, "History">;

export function HistoryScreen({ navigation }: Props): JSX.Element {
  return (
    <View style={styles.container}>
      <Button
        onPress={() => {
          navigation.dispatch(StackActions.push("List"));
        }}
      >
        2023-09-06
      </Button>
      <Button
        onPress={() => {
          navigation.dispatch(StackActions.push("List"));
        }}
      >
        2023-09-05
      </Button>
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
