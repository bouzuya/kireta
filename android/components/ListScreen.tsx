import { StackActions } from "@react-navigation/native";
import type { NativeStackScreenProps } from "@react-navigation/native-stack";
import React from "react";
import { StyleSheet, View } from "react-native";
import { Button } from "react-native-paper";
import type { NativeStackParamList } from "../types/navigation";

type Props = NativeStackScreenProps<NativeStackParamList, "List">;

export function ListScreen({ navigation }: Props): JSX.Element {
  return (
    <View style={styles.container}>
      <Button
        onPress={() => {
          navigation.dispatch(StackActions.push("Item"));
        }}
      >
        Item 1
      </Button>
      <Button
        onPress={() => {
          navigation.dispatch(StackActions.push("Item"));
        }}
      >
        Item 2
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
