import { StackActions } from "@react-navigation/native";
import type { NativeStackScreenProps } from "@react-navigation/native-stack";
import React from "react";
import { StyleSheet, View } from "react-native";
import { Button } from "react-native-paper";
import type { NativeStackParamList } from "@/types/navigation";

type Props = NativeStackScreenProps<NativeStackParamList, "Item">;

export function ItemScreen({ navigation }: Props): JSX.Element {
  return (
    <View style={styles.container}>
      <Button
        onPress={() => {
          navigation.dispatch(StackActions.push("List"));
        }}
      >
        2023-09-06
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
