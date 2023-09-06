import { StackActions, useNavigation } from "@react-navigation/native";
import React from "react";
import { StyleSheet, Text, View } from "react-native";
import { Button } from "react-native-paper";

export function ItemScreen(): JSX.Element {
  const navigation = useNavigation();
  return (
    <View style={styles.container}>
      <Text>Item Screen</Text>
      <Button onPress={() => navigation.dispatch(StackActions.pop())}>
        Back
      </Button>
      <Button
        onPress={() => navigation.dispatch(StackActions.push("ListScreen"))}
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
